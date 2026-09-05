mod auth;
mod error;
mod health;
mod leaderboard;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub(crate) use error::ApiError;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me))
        .route("/health", get(health::get))
        .route("/leaderboard", get(leaderboard::get))
}
