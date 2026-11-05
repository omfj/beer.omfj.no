use std::time::Duration;

use axum::{
    Router,
    body::Body,
    extract::MatchedPath,
    http::{
        HeaderValue, Method, Request, Response,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{Level, Span, field::Empty};

use crate::{routes, state::AppState};

/// Routes hit by uptime checks every few seconds. Their successful requests
/// are logged at debug so they don't drown out real traffic.
const QUIET_ROUTES: [&str; 2] = ["/", "/health"];

pub fn create(
    state: AppState,
    web_app_url: &str,
) -> Result<Router, axum::http::header::InvalidHeaderValue> {
    let cors_origin = web_app_url.parse::<HeaderValue>()?;

    // One span per request and exactly one log line when it finishes, with the
    // fields the Grafana dashboard queries: route, method, status, latency_ms,
    // request_id and (once authenticated) user_id.
    let trace = TraceLayer::new_for_http()
        .make_span_with(make_span)
        .on_request(())
        .on_response(on_response)
        .on_body_chunk(())
        .on_eos(())
        .on_failure(());

    Ok(Router::new()
        .merge(routes::router())
        .with_state(state)
        .layer(ServiceBuilder::new().layer(trace).layer(cors(cors_origin))))
}

fn make_span(request: &Request<Body>) -> Span {
    // The route template ("/event/{id}"), not the raw path, so ids don't
    // explode the number of distinct values.
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map_or("unmatched", MatchedPath::as_str);
    let method = request.method();
    let request_id = nanoid::nanoid!(12);

    if QUIET_ROUTES.contains(&route) {
        tracing::debug_span!("request", %method, route, %request_id, user_id = Empty)
    } else {
        tracing::info_span!("request", %method, route, %request_id, user_id = Empty)
    }
}

fn on_response(response: &Response<Body>, latency: Duration, span: &Span) {
    let status = response.status().as_u16();
    let latency_ms = u64::try_from(latency.as_millis()).unwrap_or(u64::MAX);
    // Quiet routes get a debug-level span, which is disabled (no metadata)
    // unless debug logging is on.
    let quiet = span.metadata().is_none_or(|m| *m.level() == Level::DEBUG);

    match status {
        500.. => tracing::error!(status, latency_ms, "request finished"),
        400..500 => tracing::warn!(status, latency_ms, "request finished"),
        _ if quiet => tracing::debug!(status, latency_ms, "request finished"),
        _ => tracing::info!(status, latency_ms, "request finished"),
    }
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
