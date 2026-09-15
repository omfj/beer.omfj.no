use axum::{Json, Router, routing::get};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

use super::{auth, drinks, events, health, images, leaderboard};

#[derive(OpenApi)]
#[openapi(
    info(title = "Beer"),
    paths(
        auth::register, auth::login, auth::logout, auth::me, auth::update_me,
        events::list, events::create, events::get, events::unlock,
        drinks::options, drinks::create, drinks::delete,
        images::get, health::get, leaderboard::get
    ),
    modifiers(&SessionSecurity)
)]
struct ApiDoc;

struct SessionSecurity;

impl Modify for SessionSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme(
                "session",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("auth-session"))),
            );
        // The root is an existing alias for the health endpoint.
        let mut root = openapi.paths.paths["/health"].clone();
        if let Some(operation) = root.get.as_mut() {
            operation.operation_id = Some("health_root".into());
        }
        openapi.paths.paths.insert("/".into(), root);
    }
}

pub(super) fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route(
        "/api-docs/openapi.json",
        get(|| async { Json(ApiDoc::openapi()) }),
    )
}
