// use std::time::Duration;

// use axum::{Json, extract::State, response::IntoResponse};
// use reqwest::StatusCode;
// use validator::Validate;

// use crate::{
//     app_state::AppState,
//     error::AppError,
//     modules::{
//         auth::guard::AuthUser,
//         user::dto::{CreateAddressRequest, UserProfileResponse},
//     },
//     shared::services::redis_service::RedisService,
// };

// pub async fn get_my_profile(
//     State(state): State<AppState>,
//     user: AuthUser,
// ) -> Result<impl IntoResponse, AppError> {
//     let redis_service = RedisService::new(state.redis_pool.clone());
//     let user_repo = state.user_repo;

//     let cache_key = format!("user:profile:{}", user.id);

//     //Hit cache
//     if let Ok(Some(cached_data)) = redis_service.get(&cache_key).await {
//         tracing::info!("Hit cache! Lấy profile từ cache cho user: {}", user.id);

//         // Conver String Redis -> Object UserProfile
//         if let Ok(profile) = serde_json::from_str::<UserProfileResponse>(&cached_data) {
//             return Ok((
//                 StatusCode::OK,
//                 Json(serde_json::json!({
//                     "status": "successs",
//                     "data": profile
//                 })),
//             )
//                 .into_response());
//         }
//     }

//     //No cache, find in db
//     tracing::info!("Miss Cache! Lấy profile từ Database cho user: {}", user.id);
//     let user_db = user_repo
//         .find_user_by_id(user.id)
//         .await?
//         .ok_or_else(|| AppError::NotFound("Không tìm thấy thông tin người dùng".to_string()))?;

//     let profile_response = UserProfileResponse {
//         id: user_db.id,
//         email: user_db.email,
//         name: user_db.name,
//         avatar_url: user_db.avatar_url,
//         description: user_db.description,
//         role: user_db.role,
//     };

//     //save cache
//     if let Ok(json_string) = serde_json::to_string(&profile_response) {
//         let _ = redis_service
//             .set(&cache_key, json_string, Duration::from_secs(900))
//             .await;
//     };

//     Ok((
//         StatusCode::OK,
//         Json(serde_json::json!({
//             "status": "success",
//             "data": profile_response
//         })),
//     )
//         .into_response())
// }

// pub async fn add_address(
//     State(state): State<AppState>,
//     user: AuthUser, // Lấy ID khách hàng từ Token đăng nhập
//     Json(payload): Json<CreateAddressRequest>,
// ) -> Result<impl IntoResponse, AppError> {
//     // Validate số điện thoại, tên...
//     payload
//         .validate()
//         .map_err(|e| AppError::BadRequest(format!("Lỗi dữ liệu: {}", e)))?;

//     // Giả sử vợ nhét user_service trong AppState, hoặc tự new() ở đây
//     let address_id = state
//         .user_service
//         .add_user_address(user.id, payload)
//         .await?;

//     Ok((
//         StatusCode::CREATED,
//         Json(serde_json::json!({
//             "status": "success",
//             "message": "Đã thêm địa chỉ giao hàng thành công!",
//             "data": { "address_id": address_id }
//         })),
//     )
//         .into_response())
// }
