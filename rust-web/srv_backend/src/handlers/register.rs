use crate::AppState;
use actix_web::{get, web, post, HttpRequest, HttpResponse, Responder};
use askama::Template;
use lib_error::http::ResponseError;
use lib_crypto::hash::hash;
use lib_schema::public::users::AddUser;
use lib_sql_lib::common::QueryLibrary;
use log::*;

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


#[post("/register")]
pub async fn post_register(
    _req: HttpRequest,
    state: web::Data<AppState>,
    user: web::Json<AddUser>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let hash_password = hash(&user.password)?.to_string();
    sqlx::query(&AddUser::function_call("public.insert_user($1, $2, $3)"))
        .bind(&user.name)
        .bind(&user.email)
        .bind(&hash_password)
        .execute(&db_pool)
        .await?;
    debug!("{}", format!("INSERT user: {:#?}", &user));
    Ok(HttpResponse::Ok())
}
