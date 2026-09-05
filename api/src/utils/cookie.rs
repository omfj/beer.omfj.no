use axum::http::HeaderMap;
use axum_extra::extract::cookie::CookieJar;
use cookie::{Cookie, SameSite, time::Duration};

const SESSION_COOKIE_NAME: &str = "auth-session";

pub(crate) fn session_token(headers: &HeaderMap) -> Option<String> {
    CookieJar::from_headers(headers)
        .get(SESSION_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned())
}

pub(crate) fn session_cookie(token: String, expires_at: i64, secure: bool) -> Cookie<'static> {
    let max_age = (expires_at - unix_timestamp()).max(0);
    Cookie::build((SESSION_COOKIE_NAME, token))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(same_site(secure))
        .max_age(Duration::seconds(max_age))
        .build()
}

pub(crate) fn removal_cookie(secure: bool) -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE_NAME)
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(same_site(secure))
        .build()
}

fn same_site(secure: bool) -> SameSite {
    if secure {
        SameSite::Strict
    } else {
        SameSite::Lax
    }
}

fn unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default()
}
