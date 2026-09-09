use axum::{
    Json,
    extract::{Query, State},
};
use beer_domain::time::LeaderboardYear;
use serde::Deserialize;

use crate::{routes::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct LeaderboardQuery {
    year: LeaderboardYear,
}

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<crate::services::Leaderboard>, ApiError> {
    let leaderboard = state.leaderboard.get(query.year).await?;
    Ok(Json(leaderboard))
}
