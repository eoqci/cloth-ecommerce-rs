use std::sync::Arc;

use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};

use crate::{
    config::Config,
    errors::AppError,
    modules::{
        auth::{
            AuthRepository, RotateOutcome, dto::GoogleUserInfo, error::AuthError,
            oauth::GoogleOAuthClient,
        },
        user::model::{AuthProvider, User},
    },
    shared::services::jwt::TokenService,
};

#[derive(Clone)]
pub struct AuthService {
    auth_repo: Arc<AuthRepository>,
    token_sevice: Arc<TokenService>,
    config: Arc<Config>,
    oauth_client: GoogleOAuthClient,
    http_client: oauth2::reqwest::Client,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthResult {
    pub user: User,
    pub access_token: String,
    pub raw_refresh_token: String,
}
impl AuthService {
    pub fn new(
        auth_repo: Arc<AuthRepository>,
        token_sevice: Arc<TokenService>,
        config: Arc<Config>,
        oauth_client: GoogleOAuthClient,
        http_client: oauth2::reqwest::Client,
    ) -> Self {
        Self {
            auth_repo,
            token_sevice,
            config,
            oauth_client,
            http_client,
        }
    }

    pub async fn handle_google_callback(
        &self,
        code: String,
        pkce_verifier: PkceCodeVerifier,
        user_agent: Option<&str>,
    ) -> Result<AuthResult, AppError> {
        let token_result = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.http_client)
            .await
            .map_err(|e| AppError::ExternalService {
                service: "google_oauth_token".to_string(),
                source: anyhow::anyhow!(e.to_string()),
            })?;

        let userinfo: GoogleUserInfo = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token_result.access_token().secret())
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "google_userinfo".to_string(),
                source: anyhow::anyhow!(e.to_string()),
            })?
            .json()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "google_userinfo".to_string(),
                source: anyhow::anyhow!(e.to_string()),
            })?;

        let user = self
            .auth_repo
            .find_or_create_by_google(
                &userinfo.email,
                &userinfo.name,
                userinfo.avatar_url.as_deref(),
                AuthProvider::Google,
                &userinfo.sub,
            )
            .await?;
        let raw_refresh_token = self.token_sevice.generate_refresh_token();
        let hash = &self.token_sevice.hash_refresh_token(&raw_refresh_token);

        self.auth_repo
            .create_session(
                user.id,
                &hash,
                user_agent,
                self.config.refresh_token_expired_in,
                None,
            )
            .await?;

        let access_token = self
            .token_sevice
            .generate_access_token(user.id, user.role)?;

        Ok(AuthResult {
            user,
            access_token,
            raw_refresh_token,
        })
    }

    pub async fn google_login(
        &self,
        gg_user: GoogleUserInfo,
        user_agent: Option<&str>,
    ) -> Result<TokenPair, AppError> {
        let user = self
            .auth_repo
            .find_or_create_by_google(
                &gg_user.email,
                &gg_user.name,
                gg_user.avatar_url.as_deref(),
                gg_user.provider,
                &gg_user.google_id,
            )
            .await
            .map_err(|e| {
                tracing::error!("Database Error finding/creating user: {:?}", e);
                AuthError::AuthorizationFailed
            })?;

        let access_token = self
            .token_sevice
            .generate_access_token(user.id, user.role)?;

        let refresh_token = self.token_sevice.generate_refresh_token();
        let refresh_token_hash = self.token_sevice.hash_refresh_token(&refresh_token);

        self.auth_repo
            .create_session(
                user.id,
                &refresh_token_hash,
                user_agent,
                self.config.refresh_token_expired_in,
                None,
            )
            .await
            .map_err(|_| AuthError::AuthorizationFailed)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh_token(
        &self,
        raw_refresh_token: &str,
        user_agent: Option<&str>,
    ) -> Result<TokenPair, AppError> {
        let hash = self.token_sevice.hash_refresh_token(raw_refresh_token);

        let outcome: RotateOutcome = match self.auth_repo.mark_session_used_if_valid(&hash).await? {
            Some(o) => o,
            None => {
                // cases: none exists/expired (normal) or reuse (be careful :v)
                if let Some(session) = self.auth_repo.find_session_by_token_hash(&hash).await? {
                    if session.is_used {
                        //reused - revoke whole family, not return the reason to client
                        let _ = self
                            .auth_repo
                            .revoke_session_family(session.session_family_id)
                            .await;
                    }
                }
                return Err(AppError::Unauthorized(
                    "Invalid or expired refresh token".to_string(),
                ));
            }
        };

        let user = self
            .auth_repo
            .find_user_by_id(outcome.user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        let raw_new_refresh_token = self.token_sevice.generate_refresh_token();
        let new_hash = self.token_sevice.hash_refresh_token(&raw_new_refresh_token);

        self.auth_repo
            .create_session(
                outcome.user_id,
                &new_hash,
                user_agent,
                self.config.refresh_token_expired_in,
                Some(outcome.session_family_id),
            )
            .await?;

        let access_token = self
            .token_sevice
            .generate_access_token(outcome.user_id, user.role)?;

        Ok(TokenPair {
            access_token,
            refresh_token: raw_new_refresh_token,
        })
    }

    // logout - revoke only the one (session) who out
    pub async fn logout(&self, raw_refresh_token: &str) -> Result<(), AppError> {
        let hash = self.token_sevice.hash_refresh_token(raw_refresh_token);

        if let Some(session) = self.auth_repo.find_session_by_token_hash(&hash).await? {
            self.auth_repo.revoke_session(session.id).await?;
        }

        Ok(())
    }
}
