use sqlx::PgPool;
use std::sync::Arc;

use crate::{config::Config, errors::AppError, modules::auth::state::AuthState};

#[derive(Clone)]
pub struct AppState {
    // ======================| ENV |=========================
    pub config: Arc<Config>,
    // ======================| POOL |========================
    pub db: PgPool,
    // ==================| MODULE STATE |======================
    pub auth_state: AuthState,
}

impl AppState {
    pub fn new(
        config: Arc<Config>,
        db: PgPool,
        oauth_http_client: oauth2::reqwest::Client,
        http_client: reqwest::Client,
    ) -> Result<Self, AppError> {
        //=======================================================
        // =====================| MODULE STATE |===================
        //=======================================================

        let auth_state = AuthState::new(
            db.clone(),
            oauth_http_client.clone(),
            http_client.clone(),
            config.clone(),
        )?;

        Ok(Self {
            config,
            db,
            auth_state,
        })
    }
}
