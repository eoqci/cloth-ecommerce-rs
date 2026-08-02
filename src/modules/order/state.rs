use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::order::{repository::OrderRepository, service::OrderService};

pub struct OrderState {
    pub order_repo: Arc<OrderRepository>,
    pub order_service: OrderService,
}

impl OrderState {
    pub fn new(db: PgPool) -> Self {
        let order_repo = Arc::new(OrderRepository::new(db.clone()));
        let order_service =
            OrderService::new(order_repo.clone(), cart_service, redis_service, user_repo);

        Self {
            order_repo,
            order_service,
        }
    }
}
