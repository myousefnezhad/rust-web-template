use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use lib_error::http::ResponseError;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterPage;

#[get("/register")]
pub async fn page_register(
    _req: HttpRequest,
    _state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let page = RegisterPage { };
    Ok(HttpResponse::Ok().body(page.render().unwrap()))
}
