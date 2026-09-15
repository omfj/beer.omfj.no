use axum::{
    extract::{Path, State},
    http::header,
    response::{IntoResponse, Response},
};
use beer_domain::id::ImageId;

use crate::{auth::CurrentUser, routes::ApiError, state::AppState};

#[utoipa::path(
    get,
    path = "/images/{id}",
    operation_id = "images_get",
    tag = "images",
    params(("id" = String, Path, description = "Opaque identifier")),
    responses(
        (status = 200, description = "Success", content(("image/jpeg"), ("image/png"), ("image/gif"), ("image/webp"))),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::routes::error::ErrorResponse),
        (status = 404, description = "Not found", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse),
        (status = 503, description = "Service unavailable", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path(id): Path<ImageId>,
) -> Result<Response, ApiError> {
    let image = state.drinks.image(&id, &session.user.id).await?;
    Ok((
        [
            (header::CONTENT_TYPE, image.content_type),
            (header::CACHE_CONTROL, "private, no-cache".into()),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff".into()),
        ],
        image.bytes,
    )
        .into_response())
}
