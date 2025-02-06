use actix_web::{
    dev::ServiceRequest,
    http::{header, StatusCode},
    web, Error, HttpRequest, HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use lib_app_state::AppState;
use lib_crypto::jwt::{generate_token, validate_token, Algorithm, Claims, RedisInfo};
use lib_error::http::ResponseError;
use lib_redis::Redis;
use log::*;
use serde::Serialize;

// curl -v -X POST -H 'Authorization: Bearer access_token refresh_token' -H 'Content-Type: application/json'
pub async fn validator(
    mut req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let state = match req.app_data::<web::Data<AppState>>() {
        Some(state) => state,
        None => {
            error!("Middleware: AppState not found in request!");
            return Err((
                ResponseError::new(
                    "Missing AppState".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                    -1,
                )
                .into(),
                req,
            ));
        }
    };
    let middleware_counter = state.middleware_counter.clone();
    let mut counter = middleware_counter.lock().unwrap();
    *counter += 1;
    info!("Middleware counter is uptated to {}", *counter);
    let redis = state.redis.clone();
    let config = state.app_config.clone();
    let jwt_access_key = config.jwt_access_key.clone();
    let jwt_access_session_min = config.jwt_access_session_min;
    let jwt_refresh_key = config.jwt_refresh_key.clone();

    let token: Vec<&str> = credentials.token().split(' ').collect();
    if token.len() != 2 {
        return Err((
            ResponseError::new(
                "Cannot parse tokens".to_owned(),
                StatusCode::BAD_REQUEST,
                -1,
            )
            .into(),
            req,
        ));
    }
    let access_token = token[0];
    let refresh_token = token[1];

    if let Ok(token_data) = validate_token(Algorithm::HS256, &jwt_access_key, &access_token) {
        let access_claim = token_data.claims;
        add_headers(
            &mut req,
            &access_claim,
            &format!("{} {}", &access_token, &refresh_token),
        );
        info!("User {} approved using access token", &access_claim.email);
        return Ok(req);
    } else if let Ok(token_dat) = validate_token(Algorithm::HS256, &jwt_refresh_key, &refresh_token)
    {
        // Get Refresh Token Claim
        let refresh_claim: Claims = token_dat.claims;
        // Get Redis Claim
        if let Ok(redis_info_str) = Redis::get::<String>(
            &redis,
            &format!("{}:{}", &refresh_claim.email, &refresh_claim.session),
        )
        .await
        {
            let redis_info = serde_json::from_str::<RedisInfo>(&redis_info_str);
            if let Ok(redis_info) = redis_info {
                if refresh_token == redis_info.token {
                    // Regenerate Access Token
                    let iat = Utc::now().timestamp();
                    let exp = (Utc::now() + Duration::minutes(jwt_access_session_min)).timestamp();
                    let session: u64 = refresh_claim.session;
                    let email = refresh_claim.email.to_string();
                    let access_claim = Claims {
                        iat,
                        exp,
                        email: email.clone(),
                        role: 0u64,
                        session,
                    };
                    if let Ok(access_token) =
                        generate_token(Algorithm::HS256, &jwt_access_key, access_claim)
                    {
                        add_headers(
                            &mut req,
                            &refresh_claim,
                            &format!("{} {}", &access_token, &refresh_token),
                        );
                        info!("User {} approved using refresh token", &refresh_claim.email);
                        return Ok(req);
                    }
                }
            }
        }
    }
    Err((
        ResponseError::new(
            "Access Token is not valid".to_owned(),
            StatusCode::FORBIDDEN,
            -1,
        )
        .into(),
        req,
    ))
}

fn add_headers(req: &mut ServiceRequest, claim: &Claims, tokens: &str) {
    req.headers_mut().insert(
        header::HeaderName::from_static("x-auth-email"),
        header::HeaderValue::from_bytes(&claim.email.as_bytes()).unwrap(),
    );
    req.headers_mut().insert(
        header::HeaderName::from_static("x-auth-role"),
        header::HeaderValue::from_bytes(&format!("{}", &claim.role).as_bytes()).unwrap(),
    );
    req.headers_mut().insert(
        header::HeaderName::from_static("x-auth-session"),
        header::HeaderValue::from_bytes(&format!("{}", &claim.session).as_bytes()).unwrap(),
    );
    req.headers_mut().insert(
        header::HeaderName::from_static("x-auth-tokens"),
        header::HeaderValue::from_bytes(tokens.as_bytes()).unwrap(),
    );
}

pub fn json_response<T>(json: &T, req: &HttpRequest) -> impl Responder
where
    T: Serialize,
{
    let auth_email = req
        .headers()
        .get(header::HeaderName::from_static("x-auth-email"))
        .unwrap()
        .to_str()
        .unwrap();

    let auth_role = req
        .headers()
        .get(header::HeaderName::from_static("x-auth-role"))
        .unwrap()
        .to_str()
        .unwrap();

    let auth_session = req
        .headers()
        .get(header::HeaderName::from_static("x-auth-session"))
        .unwrap()
        .to_str()
        .unwrap();

    let auth_tokens = req
        .headers()
        .get(header::HeaderName::from_static("x-auth-tokens"))
        .unwrap()
        .to_str()
        .unwrap();

    HttpResponse::Ok()
        .append_header(("x-auth-email", auth_email))
        .append_header(("x-auth-role", auth_role))
        .append_header(("x-auth-session", auth_session))
        .append_header(("x-auth-tokens", auth_tokens))
        .json(json)
}
