use crate::AppState;
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use chrono::{Duration, Utc};
use lib_crypto::{
    hash::verify,
    jwt::{generate_token, Algorithm, Claims, RedisInfo},
};
use lib_error::http::ResponseError;
use lib_redis::Redis;
use lib_schema::public::users::LoginUser;
use lib_sql_lib::common::QueryLibrary;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

#[get("/login")]
pub async fn page_login(
    _req: HttpRequest,
    _state: web::Data<AppState>,
) -> Result<impl Responder, ResponseError> {
    let page = LoginPage {};
    Ok(HttpResponse::Ok().body(page.render().unwrap()))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct LoginInfo {
    email: String,
    password: String,
    browser: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct LoginRes {
    token: String,
}

#[post("/login")]
pub async fn post_login(
    req: HttpRequest,
    state: web::Data<AppState>,
    login: web::Json<LoginInfo>,
) -> Result<impl Responder, ResponseError> {
    let pool = state.db_pool.clone();
    let config = state.app_config.clone();
    let redis = state.redis.clone();
    let jwt_access_key = config.jwt_access_key.clone();
    let jwt_access_session_min = config.jwt_access_session_min;
    let jwt_refresh_key = config.jwt_refresh_key.clone();
    let jwt_refresh_session_day = config.jwt_refresh_session_day;

    // Get Data From DB
    let user_info = match sqlx::query_as::<_, LoginUser>(&LoginUser::get_query())
        .bind(&login.email)
        .fetch_one(&pool)
        .await
    {
        Err(_) => {
            return Err(ResponseError::new(
                "User not found".to_owned(),
                StatusCode::NOT_FOUND,
                1,
            ))
        }
        Ok(res) => res,
    };

    if !verify(&login.password, &user_info.password)? {
        return Err(ResponseError::new(
            "Password is not matched".to_owned(),
            StatusCode::FORBIDDEN,
            2,
        ));
    }

    // Access Token
    let mut rng = rand::thread_rng();
    let iat = Utc::now().timestamp();
    let exp = (Utc::now() + Duration::minutes(jwt_access_session_min)).timestamp();
    let session: u64 = rng.gen();
    let email = user_info.email.to_string();
    let access_claim = Claims {
        iat,
        exp,
        email: email.clone(),
        role: 0u64,
        session,
    };

    // Refresh Token
    let exp = (Utc::now() + Duration::days(jwt_refresh_session_day)).timestamp();
    let refresh_claim = Claims {
        iat,
        exp,
        email: email.clone(),
        role: 0u64,
        session,
    };
    // Generate Tokens
    let access_token = generate_token(Algorithm::HS256, &jwt_access_key, &access_claim)?;
    let refresh_token = generate_token(Algorithm::HS256, &jwt_refresh_key, &refresh_claim)?;
    // Saving Refresh Token in Redis
    let redis_key = format!("{}:{}", &email, &session);
    let forward_address = req
        .headers()
        .get("X-Forward-For")
        .and_then(|v| v.to_str().ok());
    let address = match forward_address {
        Some(ip) => format!("{}", ip),
        None => match req.peer_addr() {
            Some(ip) => format!("{}", ip),
            None => "Not Defined!".into(),
        },
    };
    let redis_info = RedisInfo {
        email: email.clone(),
        role: 0u64,
        session,
        address,
        browser: login.browser.clone(),
        token: refresh_token.clone(),
    };
    let redis_info_str = serde_json::to_string(&redis_info)?;
    let _ = Redis::set(&redis, &redis_key, &redis_info_str).await?;
    Ok(HttpResponse::Ok().json(&LoginRes {
        token: format!("{} {}", access_token, refresh_token),
    }))
}
