use crate::AppState;
use actix_web::{delete, get, patch, post, web, HttpRequest, Responder, Result};
use lib_error::http::ResponseError;
use lib_middleware::json_response;
use lib_schema::public::groups::Groups;
use lib_sql_lib::common::QueryLibrary;
use log::*;

// curl -v -H 'Authorization: Bearer A B'  http://localhost:9000/auth/groups
#[get("/groups")]
pub async fn get_groups(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let config = state.app_config.clone();
    let res =
        sqlx::query_as::<_, Groups>(&Groups::read_query_from_lib(&config, "select_groups.sql")?)
            .fetch_all(&db_pool)
            .await?;
    debug!("GET: all groups");
    Ok(json_response(&res, &req))
}

// curl -v -X POST -H 'Content-Type: application/json' -H 'Authorization: Bearer A B' -d '{"name": "Admin"}'  http://localhost:9000/auth/group
#[post("/group")]
pub async fn post_group(
    req: HttpRequest,
    state: web::Data<AppState>,
    group: web::Json<Groups>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let _ = sqlx::query(&Groups::function_call("public.insert_group($1)"))
        .bind(group.name.clone())
        .execute(&db_pool)
        .await?;
    debug!("{}", format!("INSERT group: {:#?}", group));
    Ok(json_response::<bool>(&true, &req))
}

// curl -v -X PATCH -H 'Content-Type: application/json' -H 'Authorization: Bearer A B' -d '{"id": 2, "name": "Sale"}'  http://localhost:9000/auth/group
#[patch("/group")]
pub async fn patch_group(
    req: HttpRequest,
    state: web::Data<AppState>,
    group: web::Json<Groups>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let _ = sqlx::query(&Groups::function_call("public.update_group($1, $2)"))
        .bind(group.id.clone())
        .bind(group.name.clone())
        .execute(&db_pool)
        .await?;
    debug!("{}", format!("Update group: {:#?}", group));
    Ok(json_response::<bool>(&true, &req))
}

// curl -v -X DELETE -H 'Content-Type: application/json' -H 'Authorization: Bearer A B' -d '{"id": 1}'  http://localhost:9000/auth/group
#[delete("/group")]
pub async fn delete_group(
    req: HttpRequest,
    state: web::Data<AppState>,
    groups: web::Json<Groups>,
) -> Result<impl Responder, ResponseError> {
    let db_pool = state.db_pool.clone();
    let _ = sqlx::query(&Groups::function_call("public.delete_group($1)"))
        .bind(groups.id.clone())
        .execute(&db_pool)
        .await?;
    debug!("{}", format!("Delete group: {:#?}", groups));
    Ok(json_response::<bool>(&true, &req))
}
