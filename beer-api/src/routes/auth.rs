use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    auth::CurrentUser,
    routes::ApiError,
    services::{LoginResult, RegistrationResult, User},
    state::AppState,
    utils::cookie::{removal_cookie, session_cookie},
};
use beer_domain::{
    credentials::{Password, Username},
    profile::{Gender, Weight},
};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, utoipa::ToSchema)]
struct UserResponse {
    user: User,
}

#[derive(Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    username: String,
    password: String,
    terms_accepted: bool,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateProfileRequest {
    weight: Option<String>,
    gender: Option<String>,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    operation_id = "auth_register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Success", body = UserResponse),
        (status = 400, description = "Invalid request", body = crate::routes::error::ErrorResponse),
        (status = 409, description = "Username already taken", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    )
)]
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

#[utoipa::path(
    post,
    path = "/auth/login",
    operation_id = "auth_login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Success", body = UserResponse),
        (status = 400, description = "Invalid request", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    )
)]
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

#[utoipa::path(
    get,
    path = "/auth/me",
    operation_id = "auth_me",
    tag = "auth",
    responses(
        (status = 200, description = "Success", body = UserResponse),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
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

#[utoipa::path(
    patch,
    path = "/auth/me",
    operation_id = "auth_update_me",
    tag = "auth",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Success", body = UserResponse),
        (status = 400, description = "Invalid request", body = crate::routes::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
pub async fn update_me(
    State(state): State<AppState>,
    CurrentUser(mut session): CurrentUser,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let weight = request
        .weight
        .as_deref()
        .map(Weight::parse)
        .transpose()
        .map_err(|_| ApiError::invalid_profile())?;
    let gender = request
        .gender
        .as_deref()
        .map(Gender::parse)
        .transpose()
        .map_err(|_| ApiError::invalid_profile())?;

    state
        .auth
        .update_profile(&session.user.id, weight, gender)
        .await?;
    session.user.weight = weight;
    session.user.gender = gender;

    Ok(Json(UserResponse { user: session.user }))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    operation_id = "auth_logout",
    tag = "auth",
    responses(
        (status = 204, description = "Success"),
        (status = 401, description = "Authentication required", body = crate::routes::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::routes::error::ErrorResponse)
    ),
    security(("session" = []))
)]
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
