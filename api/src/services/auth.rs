use std::{
    fmt::Write as _,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    domain::credentials::{Password, Username},
    repositories::AuthRepository,
};

const SESSION_LIFETIME_SECONDS: i64 = 60 * 60 * 24 * 30;
const SESSION_RENEWAL_WINDOW_SECONDS: i64 = 60 * 60 * 24 * 15;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("failed to query authentication data")]
    Database(#[from] sqlx::Error),
    #[error("failed to verify password")]
    Password(#[from] bcrypt::BcryptError),
    #[error("password verification task failed")]
    PasswordTask(#[from] tokio::task::JoinError),
    #[error("system clock is before the Unix epoch")]
    Clock(#[from] std::time::SystemTimeError),
    #[error("new session could not be loaded")]
    SessionCreation,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub has_agreed_to_terms: bool,
    pub weight: Option<String>,
    pub gender: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedSession {
    pub id: String,
    pub token: String,
    pub expires_at: i64,
    pub user: User,
}

#[derive(Debug)]
pub enum LoginResult {
    InvalidCredentials,
    Authenticated(AuthenticatedSession),
}

#[derive(Clone)]
pub struct AuthService {
    repository: AuthRepository,
}

impl AuthService {
    pub fn new(repository: AuthRepository) -> Self {
        Self { repository }
    }

    pub async fn login(
        &self,
        username: &Username,
        password: &Password,
    ) -> Result<LoginResult, AuthError> {
        let Some(user) = self.repository.user_for_login(username.as_str()).await? else {
            return Ok(LoginResult::InvalidCredentials);
        };

        let password = password.as_str().to_owned();
        let password_hash = user.password_hash;
        let valid =
            tokio::task::spawn_blocking(move || bcrypt::verify(password, &password_hash)).await??;
        if !valid {
            return Ok(LoginResult::InvalidCredentials);
        }

        let token = generate_session_token();
        let id = hash_session_token(&token);
        let expires_at = now()? + SESSION_LIFETIME_SECONDS;
        self.repository
            .create_session(&id, &user.id, expires_at)
            .await?;

        let session = self
            .authenticate(&token)
            .await?
            .ok_or(AuthError::SessionCreation)?;
        Ok(LoginResult::Authenticated(session))
    }

    pub async fn authenticate(
        &self,
        token: &str,
    ) -> Result<Option<AuthenticatedSession>, AuthError> {
        let id = hash_session_token(token);
        let Some(record) = self.repository.session(&id).await? else {
            return Ok(None);
        };
        let current_time = now()?;

        if current_time >= record.expires_at {
            self.repository.delete_session(&id).await?;
            return Ok(None);
        }

        let expires_at = if current_time >= record.expires_at - SESSION_RENEWAL_WINDOW_SECONDS {
            let renewed_expiry = current_time + SESSION_LIFETIME_SECONDS;
            self.repository.renew_session(&id, renewed_expiry).await?;
            renewed_expiry
        } else {
            record.expires_at
        };

        Ok(Some(AuthenticatedSession {
            id: record.session_id,
            token: token.to_owned(),
            expires_at,
            user: User {
                id: record.user_id,
                username: record.username,
                has_agreed_to_terms: record.has_agreed_to_terms,
                weight: record.weight,
                gender: record.gender,
                created_at: record.created_at,
            },
        }))
    }

    pub async fn logout(&self, session_id: &str) -> Result<(), AuthError> {
        self.repository.delete_session(session_id).await?;
        Ok(())
    }
}

fn now() -> Result<i64, std::time::SystemTimeError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
}

fn generate_session_token() -> String {
    let mut bytes = [0_u8; 18];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn hash_session_token(token: &str) -> String {
    Sha256::digest(token.as_bytes())
        .iter()
        .fold(String::with_capacity(64), |mut hash, byte| {
            write!(hash, "{byte:02x}").expect("writing to a string cannot fail");
            hash
        })
}

#[cfg(test)]
mod tests {
    use super::{generate_session_token, hash_session_token};

    #[test]
    fn generates_svelte_compatible_session_tokens() {
        let token = generate_session_token();
        assert_eq!(token.len(), 24);
        assert!(!token.contains('='));
    }

    #[test]
    fn hashes_tokens_as_lowercase_sha256() {
        assert_eq!(
            hash_session_token("test"),
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }
}
