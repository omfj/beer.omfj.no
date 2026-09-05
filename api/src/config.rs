use std::env;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_DEVELOPMENT: bool = false;
const DEFAULT_WEB_APP_URL: &str = "http://localhost:5173";
const DEFAULT_DATABASE_URL: &str = "sqlite://beer.db";

pub struct Config {
    pub port: u16,
    pub development: bool,
    pub web_app_url: String,
    pub database_url: String,
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

        Ok(Self {
            port,
            development,
            web_app_url,
            database_url,
        })
    }
}
