use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{
    // RngExt,
    distr::{Alphanumeric, SampleString},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub email: String,
}

#[derive(Clone, Serialize)]
pub struct TokenService {
    secret: String,
    expires_in_minutes: i64,
}

impl TokenService {
    pub fn new(secret: String, expires_in_minutes: i64) -> Self {
        Self {
            secret,
            expires_in_minutes,
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid, email: String) -> Result<String, AppError> {
        let now = Utc::now();
        let expiry = now + Duration::minutes(self.expires_in_minutes);

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp() as usize,
            exp: expiry.timestamp() as usize,
            email,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
    }

    pub fn generate_refresh_token(&self) -> String {
        Alphanumeric.sample_string(&mut rand::rng(), 64)
    }

    pub fn decode_access_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::Unauthorized("Token expired".to_string())
            }
            _ => AppError::Unauthorized("Invalid Token".to_string()),
        })
    }
}
