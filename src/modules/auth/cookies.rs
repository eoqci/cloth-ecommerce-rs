use axum::response::Response;

use crate::{config::Config, shared::utils::cookies::with_cleared_cookie};

pub const ACCESS_TOKEN_COOKIE: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";
pub const CSRF_STATE_COOKIE: &str = "oauth_csrf_state";
pub const PKCE_VERIFIER_COOKIE: &str = "oauth_pkce_verifier";

pub const ACCESS_TOKEN_PATH: &str = "/";

pub const AUTH_PATH: &str = "api/v1/auth";

pub fn clear_auth_cookies(response: Response, config: &Config) -> Response {
    let response = with_cleared_cookie(response, ACCESS_TOKEN_COOKIE, ACCESS_TOKEN_PATH, config);
    with_cleared_cookie(response, REFRESH_TOKEN_COOKIE, AUTH_PATH, config)
}
