use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// curl http://localhost:9000
// https://actix.rs/docs/request
#[get("/")]
pub async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[derive(Debug, Deserialize, Serialize)]
struct HiFormData {
    name: String,
}

// curl -v http://localhost:9000/say_hi\?name\=Tony
// https://docs.rs/actix-web/4.9.0/actix_web/web/index.html
#[get("/say_hi")]
pub async fn say_hi(form_data: web::Query<HiFormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hi, {}", form_data.name))
}
