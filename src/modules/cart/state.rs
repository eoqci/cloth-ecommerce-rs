use std::sync::Arc;

use crate::modules::cart::service::CartService;

#[derive(Clone)]
pub struct CartState {
    pub cart_service: Arc<CartService>,
}
