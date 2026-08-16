use crate::{app_state::AppState, modules::auth::handler};
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/google-login", post(handler::google_login))
        .route("/google/callback", get(handler::google_callback))
        .route("/refresh-token", post(handler::refresh_token))
        .route("/logout", post(handler::logout))
}
