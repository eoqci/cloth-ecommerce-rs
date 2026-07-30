use crate::error::ConfigError;
use dotenvy::dotenv;
use email_address::EmailAddress;
use std::{env, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    //db config env
    pub database_url: String,
    //redis
    pub redis_url: String,
    //domain - host env
    pub server_host: String,
    pub server_port: u16,
    pub domain_name: String,
    // Resend env
    pub resend_api_key: String,
    pub from_email: String,
    //jwt env
    pub jwt_secret: String,
    pub jwt_expired_in: i64,
    pub refresh_token_expired_in: i32,
    //cloudflare env
    pub cf_r2_endpoint: String,
    pub cf_r2_access_key: String,
    pub cf_r2_secret_key: String,
    pub cf_r2_bucket: String,
    pub public_asset_url: String,
}

impl Config {
    fn get_env(key: &str) -> Result<String, ConfigError> {
        // Take value if not -> return empty string
        match env::var(key) {
            Ok(val) if !val.trim().is_empty() => Ok(val),
            Ok(_) => Err(ConfigError::MissingEnvVar(key.to_string())),
            Err(_) => Err(ConfigError::MissingEnvVar(key.to_string())),
        }
    }

    fn get_email_env(key: &str) -> Result<String, ConfigError> {
        let email = Self::get_env(key)?;
        EmailAddress::from_str(&email)
            .map_err(|_| ConfigError::InvalidEmailFormat(email.clone()))?;
        Ok(email)
    }

    fn get_number_env<T>(key: &str) -> Result<T, ConfigError>
    where
        T: FromStr,
    {
        let val = Self::get_env(key)?;
        val.parse::<T>()
            .map_err(|_| ConfigError::InvalidNumber(key.to_string(), val.clone()))
    }

    pub fn init() -> Result<Config, ConfigError> {
        dotenv().ok();
        Ok(Config {
            database_url: Self::get_env("DATABASE_URL")?,
            redis_url: Self::get_env("REDIS_URL")?,
            server_host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: Self::get_number_env::<u16>("PORT")?,
            domain_name: Self::get_env("DOMAIN_NAME")?,
            resend_api_key: Self::get_env("RESEND_API_KEY")?,
            from_email: Self::get_email_env("FROM_EMAIL")?,
            jwt_secret: Self::get_env("JWT_SECRET")?,
            jwt_expired_in: Self::get_number_env::<i64>("JWT_EXPIRED_IN")?,
            refresh_token_expired_in: Self::get_number_env::<i32>("REFRESH_TOKEN_EXPIRED_IN")?,
            //==============================| CLOUDFLARE |==============================
            cf_r2_access_key: Self::get_env("CF_R2_ACCESS_KEY")?,
            cf_r2_secret_key: Self::get_env("CF_R2_SECRET_KEY")?,
            cf_r2_endpoint: Self::get_env("CF_R2_ENDPOINT")?,
            cf_r2_bucket: Self::get_env("CF_R2_BUCKET")?,
            public_asset_url: Self::get_env("PUBLIC_ASSET_URL")?,
        })
    }
}
