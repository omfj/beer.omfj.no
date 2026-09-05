use crate::{auth::CurrentUser, routes::ApiError, services::events::EventLookup, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn list(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
) -> Result<Json<crate::services::Events>, ApiError> {
    Ok(Json(state.events.list(&session.user.id).await?))
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
