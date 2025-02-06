use crate::AppState;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use lib_crypto::jwt::RedisInfo;
use lib_error::http::ResponseError;
use lib_middleware::{get_email, json_response};
use lib_redis::Redis;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "session.html")]
struct SessionPage;

#[get("/session")]
pub async fn page_session(
    _req: HttpRequest,
    _state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let page = SessionPage {};
    Ok(HttpResponse::Ok().body(page.render().unwrap()))
}

#[derive(Debug, Deserialize, Serialize)]
struct SessionInfo {
    id: String,
    address: String,
    browser: String,
}

impl From<&RedisInfo> for SessionInfo {
    fn from(value: &RedisInfo) -> Self {
        Self {
            id: format!("{}", value.session),
            address: value.address.clone(),
            browser: value.browser.clone(),
        }
    }
}

#[post("/session")]
pub async fn post_session(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let redis = state.redis.clone();
    let user_email = get_email(&req);
    let redis_keys = Redis::keys(&redis, Some(&format!("{}:*", &user_email))).await?;
    let mut redis_info_str_list: Vec<String> = Vec::new();
    for key in redis_keys.iter() {
        if let Ok(redis_info_str) = Redis::get::<String>(&redis, &key).await {
            redis_info_str_list.push(redis_info_str);
        }
    }
    let mut session_info: Vec<SessionInfo> = Vec::new();
    for redis_info_str in redis_info_str_list.iter() {
        if let Ok(redis_info) = serde_json::from_str(&redis_info_str) {
            session_info.push(SessionInfo::from(&redis_info));
        }
    }
    Ok(json_response(&session_info, &req))
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteSession {
    id: String,
}

#[delete("/session")]
pub async fn delete_session(
    req: HttpRequest,
    state: web::Data<AppState>,
    input: web::Json<DeleteSession>,
) -> Result<impl Responder, ResponseError> {
    let redis = state.redis.clone();
    let user_email = get_email(&req);
    let _ = Redis::del(&redis, vec![&format!("{}:{}", &user_email, &input.id)]).await?;
    Ok(HttpResponse::Ok())
}
