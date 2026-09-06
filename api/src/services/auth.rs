use std::fmt::Write as _;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    domain::{
        credentials::{Password, Username},
        profile::{Gender, InvalidProfileValue, Weight},
    },
    repositories::AuthRepository,
    utils::{password, time::now},
};

const SESSION_LIFETIME_SECONDS: i64 = 60 * 60 * 24 * 30;
const SESSION_RENEWAL_WINDOW_SECONDS: i64 = 60 * 60 * 24 * 15;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("failed to query authentication data")]
    Database(#[from] sqlx::Error),
    #[error("failed to process password")]
    Password(#[from] password::PasswordError),
    #[error("new session could not be loaded")]
    SessionCreation,
    #[error("invalid profile value in database")]
    InvalidProfile(#[from] InvalidProfileValue),
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub has_agreed_to_terms: bool,
    pub weight: Option<Weight>,
    pub gender: Option<Gender>,
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

#[derive(Debug)]
pub enum RegistrationResult {
    UsernameTaken,
    Registered(AuthenticatedSession),
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

        let valid = password::verify(password.as_str(), &user.password_hash).await?;
        if !valid {
            return Ok(LoginResult::InvalidCredentials);
        }

        let session = self.create_authenticated_session(&user.id).await?;
        Ok(LoginResult::Authenticated(session))
    }

    pub async fn register(
        &self,
        username: &Username,
        password: &Password,
    ) -> Result<RegistrationResult, AuthError> {
        let password_hash = password::hash(password.as_str()).await?;
        let user_id = generate_user_id();

        if let Err(error) = self
            .repository
            .create_user(&user_id, username.as_str(), &password_hash)
            .await
        {
            if error
                .as_database_error()
                .is_some_and(|error| error.is_unique_violation())
            {
                return Ok(RegistrationResult::UsernameTaken);
            }
            return Err(error.into());
        }

        let session = self.create_authenticated_session(&user_id).await?;
        Ok(RegistrationResult::Registered(session))
    }

    pub async fn authenticate(
        &self,
        token: &str,
    ) -> Result<Option<AuthenticatedSession>, AuthError> {
        let id = hash_session_token(token);
        let Some(record) = self.repository.session(&id).await? else {
            return Ok(None);
        };
        let current_time = now();

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
                weight: record.weight.as_deref().map(Weight::parse).transpose()?,
                gender: record.gender.as_deref().map(Gender::parse).transpose()?,
                created_at: record.created_at,
            },
        }))
    }

    pub async fn logout(&self, session_id: &str) -> Result<(), AuthError> {
        self.repository.delete_session(session_id).await?;
        Ok(())
    }

    pub async fn update_profile(
        &self,
        user_id: &str,
        weight: Option<Weight>,
        gender: Option<Gender>,
    ) -> Result<(), AuthError> {
        self.repository
            .update_profile(
                user_id,
                weight.map(Weight::as_str),
                gender.map(Gender::as_str),
            )
            .await?;
        Ok(())
    }

    async fn create_authenticated_session(
        &self,
        user_id: &str,
    ) -> Result<AuthenticatedSession, AuthError> {
        let token = generate_session_token();
        let id = hash_session_token(&token);
        let expires_at = now() + SESSION_LIFETIME_SECONDS;
        self.repository
            .create_session(&id, user_id, expires_at)
            .await?;
        self.authenticate(&token)
            .await?
            .ok_or(AuthError::SessionCreation)
    }
}

fn generate_session_token() -> String {
    let mut bytes = [0_u8; 18];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn generate_user_id() -> String {
    const ALPHABET: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
    let mut bytes = [0_u8; 15];
    rand::rng().fill_bytes(&mut bytes);
    let mut id = String::with_capacity(24);
    let mut buffer = 0_u32;
    let mut bits = 0_u8;

    for byte in bytes {
        buffer = (buffer << 8) | u32::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            id.push(ALPHABET[((buffer >> bits) & 31) as usize] as char);
        }
    }

    id
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
    use sqlx::sqlite::SqlitePoolOptions;

    use super::{
        AuthService, RegistrationResult, generate_session_token, generate_user_id,
        hash_session_token,
    };
    use crate::{
        domain::{
            credentials::{Password, Username},
            profile::{Gender, Weight},
        },
        repositories::AuthRepository,
    };

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

    #[test]
    fn generates_svelte_compatible_user_ids() {
        let id = generate_user_id();
        assert_eq!(id.len(), 24);
        assert!(id.chars().all(|character| character.is_ascii_lowercase() || ('2'..='7').contains(&character)));
    }

    #[tokio::test]
    async fn registers_a_user_and_rejects_a_duplicate_username() {
        let database = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&database).await.unwrap();
        let service = AuthService::new(AuthRepository::new(database));
        let username = Username::parse("NewUser".into()).unwrap();
        let password = Password::parse("secret".into()).unwrap();

        let registered = service.register(&username, &password).await.unwrap();
        let RegistrationResult::Registered(session) = registered else {
            panic!("expected the first registration to succeed");
        };
        assert_eq!(session.user.username, "NewUser");
        assert!(session.user.has_agreed_to_terms);
        assert!(
            service
                .authenticate(&session.token)
                .await
                .unwrap()
                .is_some()
        );

        assert!(matches!(
            service.register(&username, &password).await.unwrap(),
            RegistrationResult::UsernameTaken
        ));
    }

    #[tokio::test]
    async fn updates_a_user_profile() {
        let database = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&database).await.unwrap();
        let service = AuthService::new(AuthRepository::new(database));
        let username = Username::parse("ProfileUser".into()).unwrap();
        let password = Password::parse("secret".into()).unwrap();
        let RegistrationResult::Registered(session) =
            service.register(&username, &password).await.unwrap()
        else {
            panic!("expected registration to succeed");
        };

        service
            .update_profile(&session.user.id, Some(Weight::Medium), Some(Gender::Other))
            .await
            .unwrap();

        let authenticated = service.authenticate(&session.token).await.unwrap().unwrap();
        assert_eq!(authenticated.user.weight, Some(Weight::Medium));
        assert_eq!(authenticated.user.gender, Some(Gender::Other));
    }
}
