use axum::{Router, routing::get};

use crate::{app_state::AppState, modules::category::handler};

pub fn category_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handler::list_categories).post(handler::create_category),
        )
        .route(
            "/{id}",
            get(handler::get_category)
                .put(handler::update_category)
                .delete(handler::delete_category),
        )
}
