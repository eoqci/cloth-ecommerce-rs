use axum::extract::FromRequestParts;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    errors::AppError,
    modules::{auth::cookies::ACCESS_TOKEN_COOKIE, user::model::UserRole},
    shared::utils::cookies::extract_cookie,
};

pub struct AuthUser {
    pub id: Uuid,
    pub role: UserRole,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let token = extract_cookie(&parts.headers, ACCESS_TOKEN_COOKIE).ok_or_else(|| {
            AppError::Unauthorized("Missing verify token. Please, login again!".to_string())
        })?;

        let claims = state.auth_state.auth_service.verify_access_token(&token)?;

        Ok(AuthUser {
            id: claims.sub,
            role: claims.role,
        })
    }
}

impl AuthUser {
    pub fn requrie_reoles(&self, allowed_roles: &[UserRole]) -> Result<(), AppError> {
        if !allowed_roles.contains(&self.role) {
            tracing::warn!(
                "Warning: Role '{:?}' Attempted unauthorized access!",
                self.role
            );
            return Err(AppError::Unauthorized(
                "You don't have any permission to access this feature!".to_string(),
            ));
        };
        Ok(())
    }
}
