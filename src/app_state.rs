use deadpool_redis::Pool;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    config::Config,
    infrastructure::{
        mail::ResendMailService, redis::client::RedisInfra, storage::r2::UploadService,
    },
    modules::{
        admin::repository::AdminRepository,
        auth::AuthRepository,
        category::repository::CategoryRepository,
        module_state::ModuleState,
        order::repository::OrderRepository,
        product::repository::ProductRepository,
        user::{repository::UserRepository, service::UserService},
    },
    shared::{ports::mail::MailService, services::jwt::TokenService},
};

#[derive(Clone)]
pub struct AppState {
    // ======================| ENV |=========================
    pub config: Arc<Config>,
    // ======================| POOL |========================
    pub db: PgPool,
    pub redis_pool: Pool,
    // ==================| SERVICE STATE |===================
    pub mail_service: Arc<dyn MailService>,
    pub token_service: Arc<TokenService>,
    pub upload_service: Arc<UploadService>,
    // =============| BUSINESS SERVICE STATE |===============
    pub user_service: Arc<UserService>,
    // ==================| REPO STATE |======================
    pub admin_repo: Arc<AdminRepository>,
    pub auth_repo: Arc<AuthRepository>,
    pub user_repo: Arc<UserRepository>,
    pub category_repo: Arc<CategoryRepository>,
    pub product_repo: Arc<ProductRepository>,
    pub order_repo: Arc<OrderRepository>,

    pub module_state: Arc<ModuleState>,
}

impl AppState {
    pub fn new(config: Config, db: PgPool) -> Self {
        let config = Arc::new(config);

        let email = format!("Shop Cua Vo <{}>", config.from_email);
        let mail_service: Arc<dyn MailService> =
            Arc::new(ResendMailService::new(config.resend_api_key.clone(), email));
        let redis_pool = RedisInfra::init_pool(&config.redis_url);
        let token_service: Arc<TokenService> = Arc::new(TokenService::new(
            config.jwt_secret.clone(),
            config.jwt_expired_in.clone(),
        ));
        //=======================================================
        // =====================| REPO STATE |===================
        //=======================================================
        let admin_repo: Arc<AdminRepository> = Arc::new(AdminRepository::new(db.clone()));
        let auth_repo: Arc<AuthRepository> = Arc::new(AuthRepository::new(db.clone()));
        let user_repo: Arc<UserRepository> = Arc::new(UserRepository::new(db.clone()));
        let category_repo: Arc<CategoryRepository> = Arc::new(CategoryRepository::new(db.clone()));
        let product_repo: Arc<ProductRepository> = Arc::new(ProductRepository::new(db.clone()));
        let order_repo: Arc<OrderRepository> = Arc::new(OrderRepository::new(db.clone()));

        //=======================================================
        // ======================| SERVICE |=====================
        //=======================================================
        let upload_service = Arc::new(UploadService::new(
            config.cf_r2_endpoint.clone(),
            config.cf_r2_access_key.clone(),
            config.cf_r2_secret_key.clone(),
            config.cf_r2_bucket.clone(),
            config.public_asset_url.clone(), // Cái tên miền images.domain.com á
        ));
        //=======================================================
        // =================| BUSINESS SERVICE |=================
        //=======================================================
        let user_service: Arc<UserService> = Arc::new(UserService::new(user_repo.clone()));
        Self {
            // Config
            config,
            db,
            // Service
            redis_pool,
            mail_service,
            upload_service,
            token_service,
            // Business Service
            user_service,
            // Repo
            admin_repo,
            auth_repo,
            user_repo,
            category_repo,
            product_repo,
            order_repo,
        }
    }
}
    }
}
