use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<i32>,
}
