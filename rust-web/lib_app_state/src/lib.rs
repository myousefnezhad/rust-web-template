use deadpool_redis::Pool as RDPool;
use lib_config::AppConfig;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub app_config: AppConfig,
    pub db_pool: Pool<Postgres>,
    pub redis: RDPool,
    pub middleware_counter: Arc<Mutex<u64>>,
}
