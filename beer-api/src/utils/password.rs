use thiserror::Error;

const BCRYPT_COST: u32 = 12;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("password operation failed")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("password task failed")]
    Task(#[from] tokio::task::JoinError),
}

pub async fn hash(password: &str) -> Result<String, PasswordError> {
    let password = password.to_owned();
    Ok(tokio::task::spawn_blocking(move || bcrypt::hash(password, BCRYPT_COST)).await??)
}

pub async fn verify(password: &str, password_hash: &str) -> Result<bool, PasswordError> {
    let password = password.to_owned();
    let password_hash = password_hash.to_owned();
    Ok(tokio::task::spawn_blocking(move || bcrypt::verify(password, &password_hash)).await??)
}
