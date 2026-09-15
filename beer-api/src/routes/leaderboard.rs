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

#[utoipa::path(
    get,
    path = "/leaderboard",
    operation_id = "leaderboard_get",
    tag = "leaderboard",
    params(("year" = i64, Query, description = "UTC calendar year")),
    responses(
        (status = 200, description = "Success", body = crate::services::Leaderboard),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    )
)]
pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<crate::services::Leaderboard>, ApiError> {
    let leaderboard = state.leaderboard.get(query.year).await?;
    Ok(Json(leaderboard))
}
