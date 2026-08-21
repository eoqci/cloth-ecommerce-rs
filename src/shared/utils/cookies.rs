use crate::config::Config;
use axum::{
    http::{HeaderMap, HeaderValue, header},
    response::Response,
};

pub fn build_cookie_string(
    name: &str,
    value: &str,
    path: &str,
    max_age: i64,
    config: &Config,
) -> String {
    // client cannot receive any cookie unless you set a domain for it

    // (my case, both my client and backend are using subdomain from one main domain,
    // so i may need it, because i cant find any better way to sovle)
    // its reading config "APP_ENV", if and had two names "production" and "development"
    let domain_attr = if config.app_env == "production" {
        format!("; Domain={}", &config.domain_name)
    } else if config.app_env == "development" {
        String::new()
    } else {
        String::new()
    };

    // secure cookies - work fine without it but only on local but on prod, we use it
    let secure_attr = if config.app_env == "production" {
        "; Secure"
    } else {
        ""
    };

    format!(
        "{}={}; Path={}{}; HttpOnly{}; SameSite=Lax; Max-Age={}",
        name, value, path, domain_attr, secure_attr, max_age
    )
}

pub fn extract_cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<String> {
    headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|c| {
                let c = c.trim();
                c.strip_prefix(&format!("{}=", name)).map(|v| v.to_string())
            })
        })
}

pub fn with_cleared_cookie(
    mut response: Response,
    name: &str,
    path: &str,
    config: &Config,
) -> Response {
    let clear = build_cookie_string(name, "", path, 0, config);
    if let Ok(v) = HeaderValue::from_str(&clear) {
        response.headers_mut().append(header::SET_COOKIE, v);
    }
    response
}

// pub fn clear_auth_cookies_response(status: StatusCode, config: &Config) -> Response {
//     // Max-Age=0
//     // Remove cookie immediately
//     let clear_access = build_cookie_string("access_token", "", "/", 0, config);
//     let clear_refresh = build_cookie_string("refresh_token", "", "/api/v1/auth", 0, config);

//     let mut response = status.into_response();
//     if let Ok(v) = HeaderValue::from_str(&clear_access) {
//         response.headers_mut().append(header::SET_COOKIE, v);
//     }
//     if let Ok(v) = HeaderValue::from_str(&clear_refresh) {
//         response.headers_mut().append(header::SET_COOKIE, v);
//     }
//     response
// }
