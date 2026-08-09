use std::sync::Arc;

use crate::{config::Config, modules::auth::AuthRepository, shared::services::jwt::TokenService};

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
}
