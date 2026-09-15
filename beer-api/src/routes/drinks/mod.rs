use beer_domain::drinks::CreatedDrink;
use beer_domain::id::{DrinkId, EventId};
mod form;

use crate::{auth::CurrentUser, routes::ApiError, services::drinks::DrinkOptions, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use axum_extra::extract::Multipart;

#[utoipa::path(
    get,
    path = "/drinks/options",
    operation_id = "drinks_options",
    tag = "drinks",
    responses(
        (status = 200, description = "Success", body = DrinkOptions),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn options(
    State(state): State<AppState>,
    CurrentUser(_): CurrentUser,
) -> Result<Json<DrinkOptions>, ApiError> {
    Ok(Json(state.drinks.options().await?))
}

#[utoipa::path(
    post,
    path = "/event/{id}/drinks",
    operation_id = "drinks_create",
    tag = "drinks",
    params(("id" = String, Path, description = "Opaque identifier")),
    request_body(content = CreateDrinkForm, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Success", body = CreatedDrink),
        (status = 400, description = "Invalid request", body = crate::routes::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::routes::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::routes::error::ErrorResponse),
        (status = 413, description = "Request body too large", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse),
        (status = 503, description = "Service unavailable", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn create(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path(id): Path<EventId>,
    multipart: Multipart,
) -> Result<(StatusCode, Json<CreatedDrink>), ApiError> {
    let user_id = &session.user.id;
    state.drinks.authorize(&id, user_id).await?;

    let input = form::parse(multipart).await?;
    let drink = state.drinks.create(&id, user_id, input).await?;

    Ok((StatusCode::CREATED, Json(drink)))
}

#[utoipa::path(
    delete,
    path = "/event/{id}/drinks/{drinkId}",
    operation_id = "drinks_delete",
    tag = "drinks",
    params(("id" = String, Path, description = "Opaque identifier"), ("drinkId" = String, Path, description = "Opaque identifier")),
    responses(
        (status = 204, description = "Success"),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::routes::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse),
        (status = 503, description = "Service unavailable", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn delete(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path((event_id, drink_id)): Path<(EventId, DrinkId)>,
) -> Result<StatusCode, ApiError> {
    state
        .drinks
        .delete(&event_id, &drink_id, &session.user.id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

// Describes the fields parsed by form::parse; the image is sent as a file part.
#[derive(utoipa::ToSchema)]
#[schema(rename_all = "camelCase")]
#[allow(dead_code)]
struct CreateDrinkForm {
    /// Image file. The entire multipart request must fit within 10 MiB.
    #[schema(value_type = String, format = Binary)]
    image: Vec<u8>,
    drink_type_id: Option<String>,
    drink_size_id: Option<String>,
    /// Alcohol percentage between 0 and 100, sent as text.
    abv: Option<String>,
}
