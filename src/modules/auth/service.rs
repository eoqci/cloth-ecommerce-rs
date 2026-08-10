use std::sync::Arc;

use crate::{
    config::Config,
    errors::AppError,
    modules::auth::{AuthRepository, dto::GoogleUserInfo, error::AuthError},
    shared::services::jwt::TokenService,
};

#[derive(Clone)]
pub struct AuthService {
    auth_repo: Arc<AuthRepository>,
    token_sevice: Arc<TokenService>,
    config: Arc<Config>,
    http_client: oauth2::reqwest::Client,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}
impl AuthService {
    pub fn new(
        auth_repo: Arc<AuthRepository>,
        token_sevice: Arc<TokenService>,
        config: Arc<Config>,
        http_client: oauth2::reqwest::Client,
    ) -> Self {
        Self {
            auth_repo,
            token_sevice,
            config,
            http_client,
        }
    }

    pub async fn google_login(
        &self,
        gg_user: GoogleUserInfo,
        user_agent: Option<&str>,
    ) -> Result<TokenPair, AppError> {
        let user = self
            .auth_repo
            .find_or_create_by_google(
                &gg_user.email,
                &gg_user.name,
                gg_user.avatar_url.as_deref(),
                gg_user.provider,
                &gg_user.google_id,
            )
            .await
            .map_err(|e| {
                tracing::error!("Database Error finding/creating user: {:?}", e);
                AuthError::AuthorizationFailed
            })?;

        let access_token = self
            .token_sevice
            .generate_access_token(user.id, user.role)?;

        let refresh_token = self.token_sevice.generate_refresh_token();

        self.auth_repo
            .create_session(
                user.id,
                &refresh_token,
                user_agent,
                self.config.refresh_token_expired_in,
                None,
            )
            .await
            .map_err(|_| AuthError::AuthorizationFailed)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
