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
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Used when `RUST_LOG` is unset.
const DEFAULT_FILTER: &str = "info";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    telemetry_init(config.development);

    let database = database::connect(&config.database_url).await?;
    let state = AppState::new(database, &config, ImageStorage::new(config.s3.as_ref())?);
    let app = router::create(state, &config.web_app_url)?;

    let address = (get_host(config.development), config.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        address = %address.0,
        port = address.1,
        "starting beer api"
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

/// Human-readable logs in development, one JSON object per line otherwise so
/// Loki/Grafana can filter and aggregate on fields.
pub fn telemetry_init(development: bool) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_FILTER));
    let registry = tracing_subscriber::registry().with(filter);

    if development {
        registry.with(tracing_subscriber::fmt::layer()).init();
    } else {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .flatten_event(true)
                    .with_current_span(true)
                    .with_span_list(false),
            )
            .init();
    }
}
