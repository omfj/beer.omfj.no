use beer_domain::drinks::CreatedDrink;
mod form;

use crate::{auth::CurrentUser, routes::ApiError, services::drinks::DrinkOptions, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use axum_extra::extract::Multipart;

pub async fn options(
    State(state): State<AppState>,
    CurrentUser(_): CurrentUser,
) -> Result<Json<DrinkOptions>, ApiError> {
    Ok(Json(state.drinks.options().await?))
}

pub async fn create(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path(id): Path<String>,
    multipart: Multipart,
) -> Result<(StatusCode, Json<CreatedDrink>), ApiError> {
    let user_id = &session.user.id;
    state.drinks.authorize(&id, user_id).await?;

    let input = form::parse(multipart).await?;
    let drink = state.drinks.create(&id, user_id, input).await?;

    Ok((StatusCode::CREATED, Json(drink)))
}

pub async fn delete(
    State(state): State<AppState>,
    CurrentUser(session): CurrentUser,
    Path((event_id, drink_id)): Path<(String, String)>,
) -> Result<StatusCode, ApiError> {
    state
        .drinks
        .delete(&event_id, &drink_id, &session.user.id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
