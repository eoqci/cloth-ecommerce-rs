use crate::modules::user::model::{AuthProvider, User, UserRole, UserSession, UserStatus};
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
        avatar_url: Option<&str>,
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

    //user session
    pub async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: &str,
        user_agent: Option<&str>,
        expires_in_day: i32,
        session_family_id: Option<Uuid>, // None = login mới, Some(id) = rotate (giữ family cũ)
    ) -> Result<UserSession, sqlx::Error> {
        sqlx::query_as!(
            UserSession,
            r#"
                INSERT INTO user_sessions (user_id, session_family_id, refresh_token_hash, user_agent, expires_at)
                VALUES ($1, COALESCE($2, gen_random_uuid()), $3, $4, NOW() + make_interval(days => $5))
                RETURNING id, user_id, session_family_id, refresh_token_hash, user_agent,
                          is_used, revoked_at, expires_at, created_at
            "#,
            user_id,
            session_family_id,
            refresh_token_hash,
            user_agent,
            expires_in_day
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Revoke toàn bộ session cùng 1 session_family_id.
    /// Gọi khi phát hiện reuse (token đã dùng bị đem dùng lại) - buộc logout cả chain.
    pub async fn revoke_session_family(&self, session_family_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                    UPDATE user_sessions
                    SET revoked_at = now()
                    WHERE session_family_id = $1 AND revoked_at IS NULL
                "#,
            session_family_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Revoke 1 session cụ thể - dùng cho "đăng xuất thiết bị này".
    pub async fn revoke_session(&self, session_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                    UPDATE user_sessions
                    SET revoked_at = now()
                    WHERE id = $1 AND revoked_at IS NULL
                "#,
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Revoke toàn bộ session của 1 user - dùng cho "đăng xuất tất cả thiết bị".
    pub async fn revoke_all_sessions_for_user(&self, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                    UPDATE user_sessions
                    SET revoked_at = now()
                    WHERE user_id = $1 AND revoked_at IS NULL
                "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Dọn session hết hạn / đã revoke lâu ngày - chạy định kỳ (cron job / tokio interval task).
    /// Session revoked giữ thêm 30 ngày trước khi xoá hẳn, phòng khi cần soi lại lịch sử (audit reuse-detection).
    pub async fn delete_expired_sessions(&self) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                    DELETE FROM user_sessions
                    WHERE expires_at < now()
                       OR (revoked_at IS NOT NULL AND revoked_at < now() - interval '30 days')
                "#
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
