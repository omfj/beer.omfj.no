mod config;
mod database;
mod domain;
mod repositories;
mod router;
mod routes;
mod services;
mod state;
mod telemetry;

use std::net::Ipv4Addr;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    telemetry::init();

    let config = Config::from_env()?;
    let database = database::connect(&config.database_url).await?;
    let state = AppState::new(database);
    let app = router::create(state, &config.web_app_url)?;

    let address = (Ipv4Addr::UNSPECIFIED, config.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(port = config.port, "API listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "failed to install shutdown signal handler");
    }
}
