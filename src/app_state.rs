use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    config::Config,
    errors::AppError,
    modules::{auth::state::AuthState, category::state::CategoryState},
};

#[derive(Clone)]
pub struct AppState {
    // ======================| ENV |=========================
    pub config: Arc<Config>,
    // ======================| POOL |========================
    pub db: PgPool,
    // ==================| MODULE STATE |======================
    pub auth_state: AuthState,
    pub category_state: CategoryState,
}

impl AppState {
    pub fn new(
        config: Arc<Config>,
        db: PgPool,
        oauth_http_client: oauth2::reqwest::Client,
        http_client: reqwest::Client,
    ) -> Result<Self, AppError> {
        //=======================================================
        //=====================| MODULE STATE |==================
        //=======================================================

        let auth_state = AuthState::new(
            db.clone(),
            oauth_http_client.clone(),
            http_client.clone(),
            config.clone(),
        )?;

        let category_state = CategoryState::new(db.clone());

        Ok(Self {
            config,
            db,

            //=======================================================
            //=====================| MODULE STATE |==================
            //=======================================================
            auth_state,
            category_state,
        })
    }
}

