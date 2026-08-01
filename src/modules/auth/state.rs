use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::auth::{AuthRepository, service::AuthService};

#[derive(Clone)]
pub struct AuthState {
    pub auth_repo: Arc<AuthRepository>,
    pub auth_service: Arc<AuthService>,
}

impl AuthState {
    pub fn new(db: PgPool) -> Self {
        let auth_repo = Arc::new(AuthRepository::new(db.clone()));
        let auth_service = Arc::new(AuthService::new(auth_repo.clone())); //just a placeholder because the auth service may need to change soon

        Self {
            auth_repo,
            auth_service,
        }
    }
}
