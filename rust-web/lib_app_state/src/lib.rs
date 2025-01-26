use deadpool_redis::Pool as RDPool;
use lib_config::AppConfig;
use sqlx::postgres::Postgres;
use sqlx::Pool;

#[derive(Clone)]
pub struct AppState {
    pub app_config: AppConfig,
    pub db_pool: Pool<Postgres>,
    pub redis: RDPool,
}
