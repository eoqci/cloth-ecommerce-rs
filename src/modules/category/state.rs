use std::sync::Arc;

use sqlx::PgPool;

<<<<<<< HEAD
use crate::modules::category::{repository::CategoryRepository, service::CategoryService};
=======
use crate::modules::category::repository::CategoryRepository;
>>>>>>> aba80b1a447ca100ad673378a6b8cfc95dc8d1aa

#[derive(Clone)]
pub struct CategoryState {
    pub category_repo: Arc<CategoryRepository>,
<<<<<<< HEAD
    pub category_service: Arc<CategoryService>,
=======
>>>>>>> aba80b1a447ca100ad673378a6b8cfc95dc8d1aa
}

impl CategoryState {
    pub fn new(db: PgPool) -> Self {
        let category_repo = Arc::new(CategoryRepository::new(db.clone()));
<<<<<<< HEAD
        let category_service = Arc::new(CategoryService::new(category_repo.clone()));
        Self {
            category_repo,
            category_service,
        }
=======

        Self { category_repo }
>>>>>>> aba80b1a447ca100ad673378a6b8cfc95dc8d1aa
    }
}
