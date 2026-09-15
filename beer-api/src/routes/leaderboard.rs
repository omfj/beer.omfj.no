use axum::{
    Json,
    extract::{Query, State},
};
use beer_domain::time::LeaderboardYear;
use serde::Deserialize;

use crate::{auth::CurrentUser, routes::ApiError, state::AppState};

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
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    CurrentUser(_): CurrentUser,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<crate::services::Leaderboard>, ApiError> {
    let leaderboard = state.leaderboard.get(query.year).await?;
    Ok(Json(leaderboard))
}
