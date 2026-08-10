use serde::{self, Deserialize};

use crate::modules::user::model::AuthProvider;

#[derive(Deserialize)]
pub struct GoogleUserInfo {
    // "sub" - because google return as "sub"
    // not google_id but we need change for readability
    #[serde(rename = "sub")]
    pub google_id: String,
    pub name: String,
    pub email: String,
    pub provider: AuthProvider,
    #[serde(rename = "picture")]
    pub avatar_url: Option<String>,
}
