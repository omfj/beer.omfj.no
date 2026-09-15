use axum::{Json, extract::State};
use serde::Serialize;

use crate::{routes::ApiError, state::AppState};

#[derive(Serialize, utoipa::ToSchema)]
pub struct Health {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "/health",
    operation_id = "health_get",
    tag = "health",
    responses(
        (status = 200, description = "Success", body = Health),
        (status = 503, description = "Service unavailable", body = crate::routes::error::ErrorResponse)
    )
)]
pub async fn get(State(state): State<AppState>) -> Result<Json<Health>, ApiError> {
    state.health.check().await?;

    Ok(Json(Health { status: "ok" }))
}
