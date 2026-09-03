use axum::{Json, extract::State};
use serde::Serialize;

use crate::{routes::ApiError, state::AppState};

#[derive(Serialize)]
pub struct Health {
    status: &'static str,
}

pub async fn get(State(state): State<AppState>) -> Result<Json<Health>, ApiError> {
    state.health.check().await?;

    Ok(Json(Health { status: "ok" }))
}
