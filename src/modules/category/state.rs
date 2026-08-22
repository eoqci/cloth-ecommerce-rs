use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{app_state::AppState, modules::category::repository::CategoryRepository};

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

impl FromRef<AppState> for Arc<CategoryRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.category_state.category_repo.clone()
    }
}
