use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::category::repository::CategoryRepository;

#[derive(Clone)]
pub struct CategoryState {
    pub category_repo: Arc<CategoryRepository>,
}

impl CategoryState {
    pub fn new(db: PgPool) -> Self {
        let category_repo = Arc::new(CategoryRepository::new(db.clone()));

        Self { category_repo }
    }
}
