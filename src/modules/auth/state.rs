use crate::{
    app_state::AppState,
    config::Config,
    errors::AppError,
    modules::auth::{
        AuthRepository,
        oauth::{GoogleOAuthClient, build_oauth_client},
        service::AuthService,
    },
    shared::services::jwt::TokenService,
};
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub auth_repo: Arc<AuthRepository>,
    pub auth_service: Arc<AuthService>,
    pub oauth_client: GoogleOAuthClient,
    pub oauth_http_client: oauth2::reqwest::Client,
}

impl AuthState {
    pub fn new(
        db: PgPool,
        oauth_http_client: oauth2::reqwest::Client,
        config: Arc<Config>,
    ) -> Result<Self, AppError> {
        let auth_repo = Arc::new(AuthRepository::new(db.clone()));
        let oauth_client = build_oauth_client(&config)?;

        let token_service = Arc::new(TokenService::new(
            config.jwt_secret.clone(),
            config.jwt_expired_in.clone(),
        ));
        let auth_service = Arc::new(AuthService::new(
            auth_repo.clone(),
            token_service,
            Arc::clone(&config),
            oauth_http_client.clone(),
        ));

        Ok(Self {
            auth_repo,
            auth_service,
            oauth_client,
            oauth_http_client,
        })
    }
}

impl FromRef<AppState> for Arc<AuthRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_state.auth_repo.clone()
    }
}

impl FromRef<AppState> for Arc<AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_state.auth_service.clone()
    }
}
