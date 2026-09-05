use std::error::Error;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

use crate::services::{AuthError, EventsError, HealthError, LeaderboardError};

type BoxError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{message}")]
    Client {
        status: StatusCode,
        code: &'static str,
        message: &'static str,
    },
    #[error("{message}")]
    Internal {
        code: &'static str,
        message: &'static str,
        #[source]
        source: BoxError,
    },
    #[error("{message}")]
    Unavailable {
        code: &'static str,
        message: &'static str,
        #[source]
        source: BoxError,
    },
}

impl ApiError {
    pub fn unauthorized() -> Self {
        Self::Client {
            status: StatusCode::UNAUTHORIZED,
            code: "unauthorized",
            message: "authentication required",
        }
    }

    pub fn invalid_credentials() -> Self {
        Self::Client {
            status: StatusCode::BAD_REQUEST,
            code: "invalid_credentials",
            message: "invalid username or password",
        }
    }

    pub fn terms_not_accepted() -> Self {
        Self::Client {
            status: StatusCode::BAD_REQUEST,
            code: "terms_not_accepted",
            message: "terms must be accepted",
        }
    }

    pub fn username_taken() -> Self {
        Self::Client {
            status: StatusCode::CONFLICT,
            code: "username_taken",
            message: "username is already taken",
        }
    }

    pub fn event_not_found() -> Self {
        Self::Client {
            status: StatusCode::NOT_FOUND,
            code: "event_not_found",
            message: "event not found",
        }
    }

    pub fn event_access_denied() -> Self {
        Self::Client {
            status: StatusCode::FORBIDDEN,
            code: "event_access_denied",
            message: "event access denied",
        }
    }

    fn internal<E>(code: &'static str, message: &'static str, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::Internal {
            code,
            message,
            source: Box::new(source),
        }
    }

    fn unavailable<E>(code: &'static str, message: &'static str, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::Unavailable {
            code,
            message,
            source: Box::new(source),
        }
    }

    fn response_parts(&self) -> (StatusCode, &'static str, &'static str) {
        match self {
            Self::Client {
                status,
                code,
                message,
            } => (*status, code, message),
            Self::Internal { code, message, .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, code, message)
            }
            Self::Unavailable { code, message, .. } => {
                (StatusCode::SERVICE_UNAVAILABLE, code, message)
            }
        }
    }
}

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        Self::internal("authentication_unavailable", "authentication failed", error)
    }
}

impl From<LeaderboardError> for ApiError {
    fn from(error: LeaderboardError) -> Self {
        Self::internal(
            "leaderboard_unavailable",
            "failed to load leaderboard",
            error,
        )
    }
}

impl From<EventsError> for ApiError {
    fn from(error: EventsError) -> Self {
        Self::internal("events_unavailable", "failed to load events", error)
    }
}

impl From<HealthError> for ApiError {
    fn from(error: HealthError) -> Self {
        Self::unavailable("database_unavailable", "database unavailable", error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = self.response_parts();

        if status.is_server_error() {
            tracing::error!(error = ?self, code, "request failed");
        }

        (
            status,
            Json(ErrorResponse {
                error: ErrorBody { code, message },
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: &'static str,
}
