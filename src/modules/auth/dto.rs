use serde::{self, Deserialize};

#[derive(Deserialize)]
pub struct GoogleUserInfo {
    // "sub" - because google return as "sub"
    // not google_id but we need change for readability
    #[serde(rename = "sub")]
    pub google_id: String,
    pub name: String,
    pub email: String,
    #[serde(rename = "picture")]
    pub avatar: Option<String>,
}
