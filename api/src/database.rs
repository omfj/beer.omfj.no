use std::str::FromStr;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use thiserror::Error;

pub type Database = SqlitePool;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("failed to parse database URL")]
    InvalidUrl(#[source] sqlx::Error),
    #[error("failed to connect to database")]
    Connection(#[source] sqlx::Error),
    #[error("failed to run database migrations")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

pub async fn connect(database_url: &str) -> Result<Database, DatabaseError> {
    let options = SqliteConnectOptions::from_str(database_url)
        .map_err(DatabaseError::InvalidUrl)?
        .create_if_missing(true);
    let database = SqlitePool::connect_with(options)
        .await
        .map_err(DatabaseError::Connection)?;

    sqlx::migrate!("./migrations").run(&database).await?;

    Ok(database)
}
