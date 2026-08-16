use std::str::FromStr;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};
use reqwest::header;

use crate::{
    app_state::AppState,
    errors::AppError,
    modules::auth::{
        cookies::{
            ACCESS_TOKEN_COOKIE, ACCESS_TOKEN_PATH, AUTH_PATH, CSRF_STATE_COOKIE,
            PKCE_VERIFIER_COOKIE, REFRESH_TOKEN_COOKIE, clear_auth_cookies,
        },
        dto::GoogleCallbackQuery,
    },
    shared::utils::cookies::{build_cookie_string, extract_cookie},
};

// [POST] /api/v1/auth/google-login - create, redirect to google
pub async fn google_login(State(state): State<AppState>) -> Response {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_token) = state
        .auth_state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
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

// [GET] api/v1/auth/google/callback
pub async fn google_callback(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<GoogleCallbackQuery>,
) -> Result<Response, AppError> {
    if let Some(err) = query.error {
        tracing::warn!(error = %err, "Google Oauth denied by user");
        return Ok(
            Redirect::to(&format!("{}/login?error=oauth_denied", state.config.fe_url))
                .into_response(),
        );
    }

    let code = query
        .code
        .ok_or_else(|| AppError::BadRequest("Missing code".to_string()))?;
    let returned_state = query
        .state
        .ok_or_else(|| AppError::BadRequest("Missing state".to_string()))?;

    // verify csrf
    let cookie_csrf = extract_cookie(&headers, CSRF_STATE_COOKIE)
        .ok_or_else(|| AppError::Unauthorized("Missing CSRF cookie".to_string()))?;
    if cookie_csrf != returned_state {
        return Err(AppError::Unauthorized("CSRF state mismatch".to_string()));
    }

    // pkce verifier
    let pkce_secret = extract_cookie(&headers, PKCE_VERIFIER_COOKIE)
        .ok_or_else(|| AppError::Unauthorized("Missing PKCE cookie".to_string()))?;
    let pkce_verifier = PkceCodeVerifier::new(pkce_secret);

    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok());

    let result = state
        .auth_state
        .auth_service
        .handle_google_callback(code, pkce_verifier, user_agent)
        .await?;

    let access_cookie = build_cookie_string(
        ACCESS_TOKEN_COOKIE,
        &result.access_token,
        ACCESS_TOKEN_PATH,
        state.config.access_token_ttl_seconds,
        &state.config,
    );

    let refresh_cookie = build_cookie_string(
        REFRESH_TOKEN_COOKIE,
        &result.raw_refresh_token,
        AUTH_PATH,
        (state.config.refresh_token_ttl_days as i64) * 86_400,
        &state.config,
    );

    let clear_csrf = build_cookie_string(CSRF_STATE_COOKIE, "", AUTH_PATH, 0, &state.config);
    let clear_pkce = build_cookie_string(PKCE_VERIFIER_COOKIE, "", AUTH_PATH, 0, &state.config);

    let mut response = Redirect::to(&state.config.fe_url).into_response();
    for cookie in [access_cookie, refresh_cookie, clear_csrf, clear_pkce] {
        if let Ok(v) = HeaderValue::from_str(&cookie) {
            response.headers_mut().append(header::SET_COOKIE, v);
        }
    }

    Ok(response)
}

// [POST] /api/v1/auth/refresh-token
pub async fn refresh_token(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok());

    let Some(raw_refresh_token) = extract_cookie(&headers, REFRESH_TOKEN_COOKIE) else {
        let resp = AppError::Unauthorized("Missing refresh token".to_string()).into_response();
        return clear_auth_cookies(resp, &state.config);
    };

    match state
        .auth_state
        .auth_service
        .refresh_token(&raw_refresh_token, user_agent)
        .await
    {
        Ok(pair) => {
            let access_cookie = build_cookie_string(
                ACCESS_TOKEN_COOKIE,
                &pair.access_token,
                ACCESS_TOKEN_PATH,
                state.config.access_token_ttl_seconds,
                &state.config,
            );
            let refresh_cookie = build_cookie_string(
                REFRESH_TOKEN_COOKIE,
                &pair.refresh_token,
                AUTH_PATH,
                (state.config.refresh_token_ttl_days as i64) * 86_400,
                &state.config,
            );

            let mut response = StatusCode::OK.into_response();
            for cookie in [access_cookie, refresh_cookie] {
                if let Ok(v) = HeaderValue::from_str(&cookie) {
                    response.headers_mut().append(header::SET_COOKIE, v);
                }
            }

            response
        }

        Err(e) => clear_auth_cookies(e.into_response(), &state.config),
    }
}

// [POST] /api/v1/auth/logout
pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    if let Some(raw_refresh_token) = extract_cookie(&headers, REFRESH_TOKEN_COOKIE) {
        // Attempt to revoke, but even if an error occurs (e.g., DB down), the client-side cookie must still be deleted—
        // to prevent the user from being stuck in a state where they "think" they've logged out while the cookie remains intact.
        if let Err(e) = state
            .auth_state
            .auth_service
            .logout(&raw_refresh_token)
            .await
        {
            tracing::warn!(error = ?e, "Failed to revoke session during logout");
        }
    }
    clear_auth_cookies(StatusCode::OK.into_response(), &state.config)
}
