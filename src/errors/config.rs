use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing enviroment varible: {0}")]
    MissingEnvVar(String),

    #[error("Invalid number for {0}: {1}")]
    InvalidNumber(String, String),

    #[error("Invalid email format: {0}")]
    InvalidEmailFormat(String),

    #[error("Environment error: {0}")]
    EnvErr(#[from] dotenvy::Error),
}
