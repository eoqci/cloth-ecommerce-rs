// use std::sync::Arc;

// use sqlx::PgPool;

// use crate::modules::user::{repository::UserRepository, service::UserService};

// pub struct UserState {
//     pub user_repo: Arc<UserRepository>,
//     pub user_service: UserService,
// }

// impl UserState {
//     pub fn new(db: PgPool) -> Self {
//         let user_repo = Arc::new(UserRepository::new(db.clone()));

//         let user_service = UserService::new(user_repo.clone());
//         Self {
//             user_repo,
//             user_service,
//         }
//     }
// }
