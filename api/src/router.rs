use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};
use tower::ServiceBuilder;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{routes, state::AppState};

pub fn create(
    state: AppState,
    web_app_url: &str,
) -> Result<Router, axum::http::header::InvalidHeaderValue> {
    let cors_origin = web_app_url.parse::<HeaderValue>()?;

    Ok(Router::new()
        .merge(routes::router())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(trace())
                .layer(cors(cors_origin)),
        ))
}

fn trace() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new())
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

fn cors(origin: HeaderValue) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            Method::OPTIONS,
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true)
}
