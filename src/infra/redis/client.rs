use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};

pub struct RedisInfra;

impl RedisInfra {
    pub fn init_pool(redis_url: &str) -> RedisPool {
        let redis_cfg = RedisConfig::from_url(redis_url);

        redis_cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Không thể khởi tạo redis Pool! Check Redis Config!")
    }
}
