hcuse std::sync::Arc;

use sqlx::PgPool;

use crate::modules::category::{repository::CategoryRepository, service::CategoryService};

#[derive(Clone)]
pub struct CategoryState {
    pub category_repo: Arc<CategoryRepository>,
    pub category_service: Arc<CategoryService>,
}

impl CategoryState {
    pub fn new(db: PgPool) -> Self {
        let category_repo = Arc::new(CategoryRepository::new(db.clone()));
        let category_service = Arc::new(CategoryService::new(category_repo.clone()));
        Self {
            category_repo,
            category_service,
        }
    }
}
