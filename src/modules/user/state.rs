use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::user::repository::UserRepository;

pub struct UserState {
    pub user_repo: Arc<UserRepository>,
}

impl UserState {
    pub fn new(db: PgPool) -> Self {
        let user_repo = Arc::new(UserRepository::new(db.clone()));

        Self { user_repo }
    }
}
