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
    pub r2_endpoint: Option<String>,
    pub r2_bucket: Option<String>,
    pub r2_access_key_id: Option<String>,
    pub r2_secret_access_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let port = env::var("PORT")
            .map(|value| value.parse())
            .unwrap_or(Ok(DEFAULT_PORT))?;
        let development = env::var("DEVELOPMENT")
            .map(|value| value.parse())
            .unwrap_or(Ok(DEFAULT_DEVELOPMENT))?;
        let web_app_url = env::var("WEB_APP_URL").unwrap_or_else(|_| DEFAULT_WEB_APP_URL.into());
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.into());

        let r2_endpoint = env::var("R2_ENDPOINT").ok();
        let r2_bucket = env::var("R2_BUCKET").ok();
        let r2_access_key_id = env::var("R2_ACCESS_KEY_ID").ok();
        let r2_secret_access_key = env::var("R2_SECRET_ACCESS_KEY").ok();
        match (&r2_endpoint, &r2_bucket, &r2_access_key_id, &r2_secret_access_key) {
            (None, None, None, None) => {},
            (Some(endpoint), Some(bucket), Some(access_key_id), Some(secret_access_key))
                if !endpoint.is_empty()
                    && !bucket.is_empty()
                    && !access_key_id.is_empty()
                    && !secret_access_key.is_empty() =>
            {}
            _ => return Err("R2_ENDPOINT, R2_BUCKET, R2_ACCESS_KEY_ID, and R2_SECRET_ACCESS_KEY must all be set and nonempty when configuring R2".into()),
        };

        Ok(Self {
            port,
            development,
            web_app_url,
            database_url,
            r2_endpoint,
            r2_bucket,
            r2_access_key_id,
            r2_secret_access_key,
        })
    }
}
