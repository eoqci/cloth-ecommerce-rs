use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::{cart::dto::CartItemDb, category::model::Category},
};

#[derive(Clone)]
pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        parent_id: Option<i32>,
    ) -> Result<Category, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
                INSERT INTO categories (name, slug, parent_id)
                VALUES ($1,$2, $3)
                RETURNING *
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(parent_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi tạo doanh mục: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(category)
    }

    pub async fn get_all(&self) -> Result<Vec<Category>, AppError> {
        // Should take a look: ORDER BY parent_id NULLS FIRST
        // This helps root categories (those without a parent) always stay at the top, making tree structure easier.
        let categories = sqlx::query_as::<_, Category>(
            "SELECT * FROM categories ORDER BY parent_id ASC NULLS FIRST, id ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi lấy danh sách danh mục: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(categories)
    }
}
