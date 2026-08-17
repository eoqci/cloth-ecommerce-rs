// use std::sync::Arc;

// use uuid::Uuid;

// use crate::{
//     error::AppError,
//     modules::user::{dto::CreateAddressRequest, repository::UserRepository},
// };

// pub struct UserService {
//     repo: Arc<UserRepository>,
// }

// impl UserService {
//     pub fn new(repo: Arc<UserRepository>) -> Self {
//         Self { repo }
//     }

//     pub async fn add_user_address(
//         &self,
//         user_id: Uuid,
//         req: CreateAddressRequest,
//     ) -> Result<Uuid, AppError> {
//         self.repo.create_address(user_id, req).await
//     }
// }
