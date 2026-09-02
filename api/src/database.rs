use std::{error::Error, str::FromStr};

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

pub type Database = SqlitePool;

pub async fn connect(database_url: &str) -> Result<Database, Box<dyn Error>> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    let database = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&database).await?;

    Ok(database)
}
