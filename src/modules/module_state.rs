use sqlx::PgPool;

use crate::{
    config::Config,
    modules::{admin::state::AdminState, auth::state::AuthState, cart::state::CartState},
};

#[derive(Clone)]
pub struct ModuleState {
    pub admin_state: AdminState,
    pub auth_state: AuthState,
    // pub user_state: UserState,
    pub cart_state: CartState,
    pub category_state: CategoryState,
    // pub product_state: ProductState,
    // pub order_state: OrderState,
}

impl ModuleState {
    pub fn new(db: PgPool, config: Config) -> Self {
        Self {
            admin_state: AdminState::new(db.clone()),
            auth_state: AuthState::new(db.clone()),
        }
    }
}
