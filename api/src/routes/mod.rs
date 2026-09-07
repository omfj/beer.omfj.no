mod auth;
mod drinks;
mod error;
mod events;
mod health;
mod images;
mod leaderboard;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
};

use crate::state::AppState;

pub(crate) use error::ApiError;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(health::get))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me).patch(auth::update_me))
        .route("/events", get(events::list).post(events::create))
        .route("/event/{id}", get(events::get))
        .route("/event/{id}/unlock", post(events::unlock))
        .route("/drinks/options", get(drinks::options))
        .route(
            "/event/{id}/drinks",
            post(drinks::create).layer(DefaultBodyLimit::max(11 * 1024 * 1024)),
        )
        .route("/event/{id}/drinks/{drinkId}", delete(drinks::delete))
        .route("/images/{id}", get(images::get))
        .route("/health", get(health::get))
        .route("/leaderboard", get(leaderboard::get))
}
