use crate::modules::admin::state::AdminState;

#[derive(Clone)]
pub struct ModuleState {
    pub admin_state: AdminState,
    pub auth_state: AuthState,
    pub user_state: UserState,
    pub cartegory_state: CartState,
    pub product_state: ProductState,
    pub order_state: OrderState,
}
