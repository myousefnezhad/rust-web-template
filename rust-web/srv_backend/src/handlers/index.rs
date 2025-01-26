use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use lib_error::http::ResponseError;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage<'a> {
    data: &'a str,
}

#[get("/")]
pub async fn page_index(
    _req: HttpRequest,
    _state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let page = IndexPage {
        data: "Data From Server ",
    };
    Ok(HttpResponse::Ok().body(page.render().unwrap()))
}
