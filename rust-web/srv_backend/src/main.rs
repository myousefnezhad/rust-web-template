use srv_backend::backend_service;

#[actix_web::main]
async fn main() {
    backend_service().await;
}
