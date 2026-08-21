use dotenvy::dotenv;
use std::{env, str::FromStr};

use crate::errors::config::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    //db config env
    pub database_url: String,
    pub redis_url: String,
    // prod - test
    pub app_env: String,
    //domain - host env
    pub fe_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub domain_name: String,
    // Resend env
    pub resend_api_key: String,
    // pub from_email: String,
    //jwt env
    pub jwt_secret: String,
    pub access_token_ttl_seconds: i64,
    pub refresh_token_ttl_days: i32,
    // google config
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
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

    // fn get_email_env(key: &str) -> Result<String, ConfigError> {
    //     let email = Self::get_env(key)?;
    //     EmailAddress::from_str(&email)
    //         .map_err(|_| ConfigError::InvalidEmailFormat(email.clone()))?;
    //     Ok(email)
    // }

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

            //==============================|  SERVER CONFIG & FE |==============================
            server_host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: Self::get_number_env::<u16>("PORT")?,
            domain_name: Self::get_env("DOMAIN_NAME")?,

            fe_url: Self::get_env("FE_URL")?,

            //==============================| EMAIL |==============================
            resend_api_key: Self::get_env("RESEND_API_KEY")?,
            // from_email: Self::get_email_env("FROM_EMAIL")?,

            //==============================| JWT |==============================
            jwt_secret: Self::get_env("JWT_SECRET")?,
            access_token_ttl_seconds: Self::get_number_env::<i64>("ACCESS_TOKEN_TTL_SECONDS")?,
            refresh_token_ttl_days: Self::get_number_env::<i32>("refresh_token_ttl_days")?,

            //==============================| GOOGLE |==============================
            google_client_id: Self::get_env("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID"),
            google_client_secret: Self::get_env("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET"),
            google_redirect_uri: Self::get_env("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI"),

            //==============================| CLOUDFLARE |==============================
            cf_r2_access_key: Self::get_env("CF_R2_ACCESS_KEY")?,
            cf_r2_secret_key: Self::get_env("CF_R2_SECRET_KEY")?,
            cf_r2_endpoint: Self::get_env("CF_R2_ENDPOINT")?,
            cf_r2_bucket: Self::get_env("CF_R2_BUCKET")?,
            public_asset_url: Self::get_env("PUBLIC_ASSET_URL")?,

            //==============================| TELEMETRY CONFIG |==============================
            app_env: Self::get_env("APP_ENV")?,
        })
    }
}
