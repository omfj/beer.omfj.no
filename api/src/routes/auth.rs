use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    auth::CurrentUser,
    domain::credentials::{Password, Username},
    routes::ApiError,
    services::{LoginResult, RegistrationResult, User},
    state::AppState,
    utils::cookie::{removal_cookie, session_cookie},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    user: User,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    username: String,
    password: String,
    terms_accepted: bool,
}

pub async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let username =
        Username::parse(request.username).map_err(|_| ApiError::invalid_credentials())?;
    let password =
        Password::parse(request.password).map_err(|_| ApiError::invalid_credentials())?;
    if !request.terms_accepted {
        return Err(ApiError::terms_not_accepted());
    }

    let session = match state.auth.register(&username, &password).await? {
        RegistrationResult::UsernameTaken => return Err(ApiError::username_taken()),
        RegistrationResult::Registered(session) => session,
    };
    let jar = jar.add(session_cookie(
        session.token,
        session.expires_at,
        state.secure_cookies,
    ));

    Ok((
        StatusCode::CREATED,
        jar,
        Json(UserResponse { user: session.user }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let username =
        Username::parse(request.username).map_err(|_| ApiError::invalid_credentials())?;
    let password =
        Password::parse(request.password).map_err(|_| ApiError::invalid_credentials())?;

    let session = match state.auth.login(&username, &password).await? {
        LoginResult::InvalidCredentials => return Err(ApiError::invalid_credentials()),
        LoginResult::Authenticated(session) => session,
    };
    let jar = jar.add(session_cookie(
        session.token,
        session.expires_at,
        state.secure_cookies,
    ));
    Ok((jar, Json(UserResponse { user: session.user })))
}

pub async fn me(
    State(state): State<AppState>,
    jar: CookieJar,
    CurrentUser(session): CurrentUser,
) -> impl IntoResponse {
    let jar = jar.add(session_cookie(
        session.token,
        session.expires_at,
        state.secure_cookies,
    ));
    (jar, Json(UserResponse { user: session.user }))
}

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
    CurrentUser(session): CurrentUser,
) -> Result<impl IntoResponse, ApiError> {
    state.auth.logout(&session.id).await?;
    Ok((
        jar.remove(removal_cookie(state.secure_cookies)),
        StatusCode::NO_CONTENT,
    ))
}
