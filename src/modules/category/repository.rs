use sqlx::PgPool;

use crate::{errors::AppError, modules::category::model::Category};

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
    ) -> Result<Category, sqlx::Error> {
        sqlx::query_as!(
            Category,
            r#"
                INSERT INTO categories (name, slug, parent_id)
                VALUES ($1, $2,$3)
                RETURNING id, name, slug, parent_id, created_at, updated_at
                "#,
            name,
            slug,
            parent_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as!(
            Category,
            r#"
                SELECT id, name, slug, parent_id, created_at, updated_at FROM categories WHERE id = $1
            "#
            , id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as!(
            Category,
            r#"
                SELECT id, name, slug, parent_id, created_at, updated_at FROM categories WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list_all(&self) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as!(
            Category,
            r#"
                SELECT id, name, slug, parent_id, created_at, updated_at FROM categories ORDER BY name
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    // Update name, slug, and parent_id. When changing parent_id, automatically check for and prevent multi-level circular references –
    // The database CHECK constraint only prevents a record from being its own parent; it cannot prevent A -> B -> A scenarios.
    pub async fn update(
        &self,
        id: i32,
        name: &str,
        slug: &str,
        parent_id: Option<i32>,
    ) -> Result<Category, AppError> {
        if let Some(new_parent_id) = parent_id {
            if new_parent_id == id {
                return Err(AppError::BadRequest(
                    "A category cannot be its own parent.".to_string(),
                ));
            }

            let would_cycle = sqlx::query_scalar!(
                r#"
                            WITH RECURSIVE ancestors AS (
                                SELECT id, parent_id FROM categories WHERE id = $1
                                UNION ALL
                                SELECT c.id, c.parent_id FROM categories c
                                INNER JOIN ancestors a ON c.id = a.parent_id
                            )
                            SELECT EXISTS (SELECT 1 FROM ancestors WHERE id = $2) AS "would_cycle!"
                        "#,
                new_parent_id,
                id
            )
            .fetch_one(&self.pool)
            .await?;
            if would_cycle {
                return Err(AppError::BadRequest(
                    "A subcategory cannot be set as the parent of itself (creating a loop)"
                        .to_string(),
                ));
            }
        }

        sqlx::query_as!(
            Category,
            r#"
                            UPDATE categories
                            SET name = $1, slug = $2, parent_id = $3
                            WHERE id = $4
                            RETURNING id, name, slug, parent_id, created_at, updated_at
                        "#,
            name,
            slug,
            parent_id,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM categories WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn create_and_find_category(pool: PgPool) {
        let repo = CategoryRepository::new(pool);

        let created = repo.create("Áo", "ao", None).await.unwrap();
        assert_eq!(created.name, "Áo");
        assert!(created.parent_id.is_none());

        let found = repo.find_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(found.id, created.id);

        let by_slug = repo.find_by_slug("ao").await.unwrap().unwrap();
        assert_eq!(by_slug.id, created.id);
    }

    #[sqlx::test]
    async fn update_rejects_self_parent(pool: PgPool) {
        let repo = CategoryRepository::new(pool);
        let a = repo.create("A", "a", None).await.unwrap();

        let result = repo.update(a.id, "A", "a", Some(a.id)).await;
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[sqlx::test]
    async fn update_rejects_multi_level_cycle(pool: PgPool) {
        let repo = CategoryRepository::new(pool);
        let a = repo.create("A", "a", None).await.unwrap();
        let b = repo.create("B", "b", Some(a.id)).await.unwrap();
        let c = repo.create("C", "c", Some(b.id)).await.unwrap();

        // A -> B -> C, thử đặt cha của A thành C (chính là cháu của A) - phải bị chặn
        let result = repo.update(a.id, "A", "a", Some(c.id)).await;
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[sqlx::test]
    async fn update_allows_valid_reparent(pool: PgPool) {
        let repo = CategoryRepository::new(pool);
        let a = repo.create("A", "a", None).await.unwrap();
        let b = repo.create("B", "b", Some(a.id)).await.unwrap();
        let d = repo.create("D", "d", None).await.unwrap();

        // D đang không có cha, đổi sang làm con của B - hợp lệ, không cycle
        let updated = repo.update(d.id, "D", "d", Some(b.id)).await.unwrap();
        assert_eq!(updated.parent_id, Some(b.id));
    }
}
