use axum::{
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Redirect},
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use reqwest::header;

use crate::{
    app_state::AppState,
    modules::auth::cookies::{AUTH_PATH, CSRF_STATE_COOKIE, PKCE_VERIFIER_COOKIE},
    shared::utils::cookies::build_cookie_string,
};

// [POST] /api/v1/auth/google -create, redirect to google
pub async fn google_login(State(state): State<AppState>) -> Response {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_token) = state
        .auth_state
        .oauth_client
        .authorize_url(CsrfToken::new_random())
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let csrf_cookie = build_cookie_string(
        CSRF_STATE_COOKIE,
        csrf_token.secret(),
        AUTH_PATH,
        600, // 10 min
        &state.config,
    );

    let pkce_cookie = build_cookie_string(
        PKCE_VERIFIER_COOKIE,
        pkce_verifier.secret(),
        AUTH_PATH,
        600,
        &state.config,
    );

    let mut response = Redirect::to(authorize_url.as_str()).into_response();
    for cookie in [csrf_cookie, pkce_cookie] {
        if let Ok(v) = HeaderValue::from_str(&cookie) {
            response.headers_mut().append(header::SET_COOKIE, v);
        }
    }

    response
}
