use crate::AppState;
use actix_web::{delete, get, patch, post, web, HttpRequest, Responder, Result};
use lib_error::http::ResponseError;
use lib_middleware::json_response;
use lib_schema::public::users::{AddUser, DeleteUser, ListUser, UpdateUser};
use log::*;

// curl -v -H 'Authorization: Bearer A B'  http://localhost:9000/auth/users
#[get("/users")]
pub async fn get_users(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let res = sqlx::query_as::<_, ListUser>("SELECT * FROM public.users")
        .fetch_all(&db_pool)
        .await?;
    debug!("GET: all users");
    Ok(json_response(&res, &req))
}

// curl -v -X POST -H 'Content-Type: application/json' -H 'Authorization: Bearer A B' -d '{"name": "Tony", "email": "tony@mail.com", "password": "1234"}'  http://localhost:9000/auth/user
#[post("/user")]
pub async fn post_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    user: web::Json<AddUser>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let _ = sqlx::query("INSERT INTO public.users (name, email, password) VALUES ($1, $2, $3)")
        .bind(user.name.clone())
        .bind(user.email.clone())
        .bind(user.password.clone())
        .execute(&db_pool)
        .await?;
    debug!("{}", format!("INSERT user: {:#?}", user));
    Ok(json_response::<AddUser>(&user, &req))
}

// curl -v -X PATCH -H 'Content-Type: application/json' -H 'Authorization: Bearer A B' -d '{"id": 2, "name": "Alex", "email": "alex@mail.com", "password": "876"}'  http://localhost:9000/auth/user
#[patch("/user")]
pub async fn patch_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    user: web::Json<UpdateUser>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let _ = sqlx::query("UPDATE public.users SET name=$1, email=$2, password=$3 WHERE id=$4")
        .bind(user.name.clone())
        .bind(user.email.clone())
        .bind(user.password.clone())
        .bind(user.id.clone())
        .execute(&db_pool)
        .await?;
    debug!("{}", format!("Update user: {:#?}", user));
    Ok(json_response::<UpdateUser>(&user, &req))
}

// curl -v -X DELETE -H 'Content-Type: application/json' -H 'Authorization: Bearer A B' -d '{"id": 1}'  http://localhost:9000/auth/user
#[delete("/user")]
pub async fn delete_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    user: web::Json<DeleteUser>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let _ = sqlx::query("DELETE FROM public.users WHERE id=$1")
        .bind(user.id.clone())
        .execute(&db_pool)
        .await?;
    debug!("{}", format!("Delete user: {:#?}", user));
    Ok(json_response::<DeleteUser>(&user, &req))
}
