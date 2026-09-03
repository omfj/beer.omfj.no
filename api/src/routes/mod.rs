mod error;
mod health;
mod leaderboard;

use axum::{Router, routing::get};

use crate::state::AppState;

use error::ApiError;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::get))
        .route("/leaderboard", get(leaderboard::get))
}
