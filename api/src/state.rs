use std::str::FromStr;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
        let db = SqlitePool::connect_with(options).await?;

        Ok(Self { db })
    }
}
