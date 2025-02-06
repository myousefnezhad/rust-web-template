use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use lib_error::http::ResponseError;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage<'a> {
    data: &'a str,
    counter: u64,
}

#[get("/")]
pub async fn page_index(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let middleware_counter = state.middleware_counter.clone();
    let counter = *middleware_counter.lock().unwrap();
    let page = IndexPage {
        data: "Data From Server ",
        counter,
    };
    Ok(HttpResponse::Ok().body(page.render().unwrap()))
}
