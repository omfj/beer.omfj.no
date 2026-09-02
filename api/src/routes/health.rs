use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct Health {
    status: &'static str,
}

pub async fn get(
    State(state): State<AppState>,
) -> Result<Json<Health>, (StatusCode, &'static str)> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|error| {
            tracing::error!(%error, "database health check failed");
            (StatusCode::SERVICE_UNAVAILABLE, "database unavailable")
        })?;

    Ok(Json(Health { status: "ok" }))
}
