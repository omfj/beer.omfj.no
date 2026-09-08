use crate::{
    routes::ApiError, services::AuthenticatedSession, state::AppState, utils::cookie::session_token,
};
use axum::{extract::FromRequestParts, http::request::Parts};

pub struct CurrentUser(pub AuthenticatedSession);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = session_token(&parts.headers).ok_or_else(ApiError::unauthorized)?;
        state
            .auth
            .authenticate(&token)
            .await?
            .map(Self)
            .ok_or_else(ApiError::unauthorized)
    }
}
