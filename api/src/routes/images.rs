use axum::{
    extract::{Path, State},
    http::header,
    response::{IntoResponse, Response},
};

use crate::{auth::CurrentUser, routes::ApiError, state::AppState};

pub async fn get(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path(id): Path<String>,
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
