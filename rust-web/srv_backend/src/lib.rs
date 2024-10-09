mod handlers;
extern crate ansi_escapes;
use crate::handlers::handlers;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use lib_config::AppConfig;
use log::*;
use std::thread::sleep;
use dotenv::dotenv;
use std::time::Duration;
use std::env;
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;


const JSON_REQUEST_SIZE: usize = 1024 * 1024 * 50; // 50 Mb
const PAYL_REQUEST_SIZE: usize = 1024 * 1024; // 1 Mb
const KEEP_ALIVE: u64 = 60; // 1 min

#[derive(Debug, Clone)]
pub struct AppState {
    pub app_config: AppConfig,
    pub db_pool: Pool<Postgres>,
}


pub async fn backend_service() {
    dotenv().ok();

    let app_config = AppConfig::new();
    let app_bind = app_config.bind_backend.clone();
    let app_workers = app_config.workers_backend;
    let pg_connection = app_config.pg_connection;

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            debug!("{}", e);
            panic!("Cannot locate DATABASE_URL env variable");
        }
    };

    let db_pool = PgPoolOptions::new()
        .max_connections(pg_connection.try_into().unwrap())
        .connect(&database_url).await.unwrap();

    let app_state = AppState {
        app_config: app_config.clone(),
        db_pool: db_pool,
    };

    env_logger::init_from_env(Env::default().default_filter_or(app_config.log_level.clone()));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::JsonConfig::default().limit(JSON_REQUEST_SIZE))
            .app_data(web::PayloadConfig::default().limit(PAYL_REQUEST_SIZE))
            .configure(handlers)
    })
    .workers(app_workers)
    .keep_alive(Duration::from_secs(KEEP_ALIVE))
    .bind(&app_bind)
    .unwrap()
    .run();
    info!(
        r#"
Server is live. 
######## SERVER INFORMATION ########
BIND {}
######## SERVER INFORMATION ########
CTRL+C to exit."#,
        &app_bind
    );
    println!("{}", ansi_escapes::Beep);
    sleep(Duration::from_millis(200));
    println!("{}", ansi_escapes::Beep);
    server.await.unwrap();
}
