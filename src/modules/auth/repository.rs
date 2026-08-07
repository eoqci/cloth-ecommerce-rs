use crate::modules::user::model::{AuthProvider, User, UserRole, UserStatus};
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthRepository {
    pool: PgPool,
}

pub struct OtpRecord {
    pub id: uuid::Uuid,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl AuthRepository {
    // Create Func
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // find or create by google - oauth
    pub async fn find_or_create_by_google(
        &self,
        email: &str,
        name: &str,
        avatar_url: &str,
        provider: AuthProvider,
        provider_id: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                    INSERT INTO users (email, name, avatar_url, provider, provider_id)
                    VALUES (lower($1), $2, $3, $4, $5)
                    ON CONFLICT (lower(email)) DO UPDATE
                    SET
                        provider_id = EXCLUDED.provider_id,
                        name        = EXCLUDED.name,
                        avatar_url  = EXCLUDED.avatar_url,
                        updated_at  = now()
                    RETURNING
                        id,
                        email,
                        name,
                        avatar_url,
                        description,
                        role AS "role: UserRole",
                        status AS "status: UserStatus",
                        provider AS "provider: AuthProvider",
                        provider_id,
                        created_at,
                        updated_at
                "#,
            email,
            name,
            avatar_url,
            provider as AuthProvider, // Passed explicitly for the macro
            provider_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
                    SELECT
                        id,
                        email,
                        name,
                        avatar_url,
                        description,
                        role AS "role: UserRole",
                        status AS "status: UserStatus",
                        provider AS "provider: AuthProvider",
                        provider_id,
                        created_at,
                        updated_at
                    FROM users
                    WHERE id = $1
                "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
                    SELECT
                        id,
                        email,
                        name,
                        avatar_url,
                        description,
                        role AS "role: UserRole",
                        status AS "status: UserStatus",
                        provider AS "provider: AuthProvider",
                        provider_id,
                        created_at,
                        updated_at
                    FROM users
                    WHERE lower(email) = lower($1)
                "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
    }
}
