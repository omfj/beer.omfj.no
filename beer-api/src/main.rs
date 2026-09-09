mod auth;
mod config;
mod database;
mod repositories;
mod router;
mod routes;
mod services;
mod state;
mod utils;

use std::net::Ipv4Addr;

use beer_storage::ImageStorage;
use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::telemetry::init();

    let config = Config::from_env()?;
    let database = database::connect(&config.database_url).await?;
    let state = AppState::new(database, &config, ImageStorage::new(config.s3.as_ref())?);
    let app = router::create(state, &config.web_app_url)?;

    let address = (get_host(config.development), config.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("🚀 Starting Beer API");
    tracing::info!(
        "🚀 Running on http://{}:{}",
        address.0.to_string(),
        address.1
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn get_host(development: bool) -> Ipv4Addr {
    if development {
        Ipv4Addr::LOCALHOST
    } else {
        Ipv4Addr::UNSPECIFIED
    }
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "failed to install shutdown signal handler");
    }
}
