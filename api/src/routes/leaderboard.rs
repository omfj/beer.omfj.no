use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{routes::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct LeaderboardQuery {
    year: i64,
}

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<crate::services::Leaderboard>, ApiError> {
    let leaderboard = state.leaderboard.get(query.year).await?;
    Ok(Json(leaderboard))
}
