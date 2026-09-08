use std::env;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_DEVELOPMENT: bool = false;
const DEFAULT_WEB_APP_URL: &str = "http://localhost:5173";
const DEFAULT_DATABASE_URL: &str = "sqlite://dev.db";
const INVALID_S3_CONFIG: &str = "S3_ENDPOINT, S3_BUCKET, S3_ACCESS_KEY_ID, and S3_SECRET_ACCESS_KEY must all be set and nonempty when configuring S3";

pub struct Config {
    pub port: u16,
    pub development: bool,
    pub web_app_url: String,
    pub database_url: String,
    pub s3: Option<S3Config>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let port = env::var("PORT")
            .map(|value| value.parse())
            .unwrap_or(Ok(DEFAULT_PORT))?;
        let development = env::var("DEVELOPMENT")
            .map(|value| value.parse())
            .unwrap_or(Ok(DEFAULT_DEVELOPMENT))?;
        let web_app_url = env::var("WEB_APP_URL").unwrap_or_else(|_| DEFAULT_WEB_APP_URL.into());
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.into());

        Ok(Self {
            port,
            development,
            web_app_url,
            database_url,
            s3: S3Config::load()?,
        })
    }
}

pub struct S3Config {
    pub endpoint: String,
    pub bucket: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

impl S3Config {
    pub fn load() -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let values = [
            "S3_ENDPOINT",
            "S3_BUCKET",
            "S3_ACCESS_KEY_ID",
            "S3_SECRET_ACCESS_KEY",
        ]
        .map(|name| env::var(name).ok());

        if values.iter().all(Option::is_none) {
            return Ok(None);
        }

        let [
            Some(endpoint),
            Some(bucket),
            Some(access_key_id),
            Some(secret_access_key),
        ] = values
        else {
            return Err(INVALID_S3_CONFIG.into());
        };

        if [&endpoint, &bucket, &access_key_id, &secret_access_key]
            .iter()
            .any(|value| value.is_empty())
        {
            return Err(INVALID_S3_CONFIG.into());
        }

        Ok(Some(Self {
            endpoint,
            bucket,
            access_key_id,
            secret_access_key,
        }))
    }
}
