use std::env;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_DEVELOPMENT: bool = false;
const DEFAULT_WEB_APP_URL: &str = "http://localhost:5173";
const DEFAULT_DATABASE_URL: &str = "sqlite://dev.db";

pub struct Config {
    pub port: u16,
    pub development: bool,
    pub web_app_url: String,
    pub database_url: String,
    pub s3_endpoint: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_access_key_id: Option<String>,
    pub s3_secret_access_key: Option<String>,
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

        let s3_endpoint = env::var("S3_ENDPOINT").ok();
        let s3_bucket = env::var("S3_BUCKET").ok();
        let s3_access_key_id = env::var("S3_ACCESS_KEY_ID").ok();
        let s3_secret_access_key = env::var("S3_SECRET_ACCESS_KEY").ok();
        match (&s3_endpoint, &s3_bucket, &s3_access_key_id, &s3_secret_access_key) {
            (None, None, None, None) => {},
            (Some(endpoint), Some(bucket), Some(access_key_id), Some(secret_access_key))
                if !endpoint.is_empty()
                    && !bucket.is_empty()
                    && !access_key_id.is_empty()
                    && !secret_access_key.is_empty() =>
            {}
            _ => return Err("S3_ENDPOINT, S3_BUCKET, S3_ACCESS_KEY_ID, and S3_SECRET_ACCESS_KEY must all be set and nonempty when configuring S3".into()),
        };

        Ok(Self {
            port,
            development,
            web_app_url,
            database_url,
            s3_endpoint,
            s3_bucket,
            s3_access_key_id,
            s3_secret_access_key,
        })
    }
}
