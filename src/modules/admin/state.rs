use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::admin::{repository::AdminRepository, service::AdminService};

#[derive(Clone)]
pub struct AdminState {
    pub admin_repo: Arc<AdminRepository>,
    pub admin_service: Arc<AdminService>,
}

impl AdminState {
    pub fn new(db: PgPool) -> Self {
        let admin_repo = Arc::new(AdminRepository::new(db.clone()));
        let admin_service = Arc::new(AdminService::new(admin_repo.clone()));

        Self {
            admin_repo,
            admin_service,
        }
    }
}
