use crate::{errors::AppError, modules::user::model::UserRole};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{
    // RngExt,
    distr::{Alphanumeric, SampleString},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone, Serialize)]
pub struct TokenService {
    secret: String,
    // RFC 7519 - standard (Unix timestamp count as second)
    expires_in_seconds: i64,
}

impl TokenService {
    pub fn new(secret: String, expires_in_seconds: i64) -> Self {
        Self {
            secret,
            expires_in_seconds,
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid, role: UserRole) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            role,
            iat: now.timestamp(),
            exp: (now + Duration::seconds(self.expires_in_seconds)).timestamp(),
        };

        encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
    }

    pub fn generate_refresh_token(&self) -> String {
        Alphanumeric.sample_string(&mut rand::rng(), 64)
    }

    pub fn hash_refresh_token(&self, raw_token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw_token.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))
    }
}
