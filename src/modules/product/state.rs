use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::product::{repository::ProductRepository, service::ProductService};

pub struct ProductState {
    pub product_repo: Arc<ProductRepository>,
    pub product_service: ProductService,
}

impl ProductState {
    pub fn new(db: PgPool) -> Self {
        let product_repo = Arc::new(ProductRepository::new(db.clone()));
        let product_service = ProductService::new(product_repo.clone());

        Self {
            product_repo,
            product_service,
        }
    }
}
