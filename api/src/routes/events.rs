use crate::{auth::CurrentUser, routes::ApiError, services::events::EventLookup, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateEventRequest {
    name: String,
    password: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
) -> Result<Json<crate::services::Events>, ApiError> {
    Ok(Json(state.events.list(&session.user.id).await?))
}

pub async fn create(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Json(request): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<crate::services::CreatedEvent>), ApiError> {
    if request.name.is_empty() {
        return Err(ApiError::invalid_event_name());
    }
    let password = request
        .password
        .as_deref()
        .filter(|value| !value.is_empty());
    let event = state
        .events
        .create(&request.name, password, &session.user.id)
        .await?;
    Ok((StatusCode::CREATED, Json(event)))
}

pub async fn get(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path(id): Path<String>,
) -> Result<Json<crate::services::EventDetail>, ApiError> {
    match state.events.get(&id, &session.user.id).await? {
        EventLookup::Found(event) => Ok(Json(event)),
        EventLookup::NotFound => Err(ApiError::event_not_found()),
        EventLookup::Forbidden => Err(ApiError::event_access_denied()),
    }
}
