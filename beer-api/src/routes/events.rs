use crate::{
    auth::CurrentUser,
    routes::ApiError,
    services::events::{EventLookup, UnlockResult},
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use beer_domain::id::EventId;
use serde::Deserialize;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateEventRequest {
    name: String,
    password: Option<String>,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UnlockEventRequest {
    password: String,
}

#[utoipa::path(
    get,
    path = "/events",
    operation_id = "events_list",
    tag = "events",
    responses(
        (status = 200, description = "Success", body = crate::services::Events),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn list(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
) -> Result<Json<crate::services::Events>, ApiError> {
    Ok(Json(state.events.list(&session.user.id).await?))
}

#[utoipa::path(
    post,
    path = "/events",
    operation_id = "events_create",
    tag = "events",
    request_body = CreateEventRequest,
    responses(
        (status = 201, description = "Success", body = crate::services::CreatedEvent),
        (status = 400, description = "Invalid request", body = crate::routes::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
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

#[utoipa::path(
    get,
    path = "/event/{id}",
    operation_id = "events_get",
    tag = "events",
    params(("id" = String, Path, description = "Opaque identifier")),
    responses(
        (status = 200, description = "Success", body = crate::services::EventDetail),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::routes::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path(id): Path<EventId>,
) -> Result<Json<crate::services::EventDetail>, ApiError> {
    match state.events.get(&id, &session.user.id).await? {
        EventLookup::Found(event) => Ok(Json(event)),
        EventLookup::NotFound => Err(ApiError::event_not_found()),
        EventLookup::Forbidden => Err(ApiError::event_access_denied()),
    }
}

#[utoipa::path(
    post,
    path = "/event/{id}/unlock",
    operation_id = "events_unlock",
    tag = "events",
    params(("id" = String, Path, description = "Opaque identifier")),
    request_body = UnlockEventRequest,
    responses(
        (status = 204, description = "Success"),
        (status = 400, description = "Invalid request", body = crate::routes::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn unlock(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path(id): Path<EventId>,
    Json(request): Json<UnlockEventRequest>,
) -> Result<StatusCode, ApiError> {
    match state
        .events
        .unlock(&id, &session.user.id, &request.password)
        .await?
    {
        UnlockResult::NotFound => Err(ApiError::event_not_found()),
        UnlockResult::InvalidPassword => Err(ApiError::invalid_event_password()),
        UnlockResult::Unlocked => Ok(StatusCode::NO_CONTENT),
    }
}
