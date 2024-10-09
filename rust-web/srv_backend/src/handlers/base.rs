use actix_web::{get, HttpResponse, Responder};

// curl http://localhost:9000
#[get("/")]
pub async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
