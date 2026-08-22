use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    errors::AppError,
    modules::{
        auth::guard::AuthUser,
        category::{
            dto::{CreateCategoryRequest, UpdateCategoryRequest},
            model::Category,
            repository::CategoryRepository,
        },
        user::model::UserRole,
    },
};

pub async fn list_categories(
    State(state): State<Arc<CategoryRepository>>,
) -> Result<Json<Vec<Category>>, AppError> {
    Ok(Json(state.list_all().await?))
}

pub async fn get_category(
    State(state): State<Arc<CategoryRepository>>,
    Path(id): Path<i32>,
) -> Result<Json<Category>, AppError> {
    let category = state
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound {
            resource: "Category".to_string(),
        })?;
    Ok(Json(category))
}

// Admin Only
pub async fn create_category(
    user: AuthUser,
    State(state): State<Arc<CategoryRepository>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    user.require_roles(&[UserRole::Admin, UserRole::Moderator])?;
    let category = state
        .create(&payload.name, &payload.slug, payload.parent_id)
        .await?;
    Ok(Json(category))
}

pub async fn update_category(
    user: AuthUser,
    State(state): State<Arc<CategoryRepository>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    user.require_roles(&[UserRole::Admin, UserRole::Moderator])?;
    let category = state
        .update(id, &payload.name, &payload.slug, payload.parent_id)
        .await?;
    Ok(Json(category))
}

pub async fn delete_category(
    user: AuthUser,
    State(state): State<Arc<CategoryRepository>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    user.require_roles(&[UserRole::Admin, UserRole::Moderator])?;
    state.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
