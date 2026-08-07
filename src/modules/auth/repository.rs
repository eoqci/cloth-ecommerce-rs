use crate::modules::user::model::{AuthProvider, User, UserRole, UserStatus};
use sqlx::{Error, PgPool};

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
        .fetch_one(&self.pool) // Make sure to replace `&self.pool` with your actual connection pool field
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT
                    id,
                    email,
                    password_hash,
                    name,
                    avatar_url,
                    description,
                    role as "role: UserRole",
                    status as "status: UserStatus",
                    provider as "provider: AuthProvider",
                    provider_id,
                    created_at,
                    updated_at
                FROM users
                WHERE email = $1
                "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn active_user(&self, user_id: uuid::Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
                UPDATE users
                SET status = 'active'::user_status_type, updated_at = NOW()
                WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_session(
        &self,
        user_id: uuid::Uuid,
        refresh_token: &str,
        user_agent: &str,
        expires_in_day: i32,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
                INSERT INTO user_sessions (user_id, refresh_token, user_agent, expires_at)
                VALUES ($1, $2, $3, NOW() + make_interval(days => $4))
            "#,
            user_id,
            refresh_token,
            user_agent,
            expires_in_day
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
