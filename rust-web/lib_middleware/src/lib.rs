use actix_web::{
    dev::ServiceRequest,
    http::{header, StatusCode},
    web, Error, HttpRequest, HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use lib_app_state::AppState;
use lib_crypto::jwt::{validate_token, Algorithm, Claims};
use lib_error::http::ResponseError;
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

    let _redis = state.redis.clone();
    let config = state.app_config.clone();
    let jwt_access_key = config.jwt_access_key.clone();
    let _jwt_refresh_key = config.jwt_refresh_key.clone();

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
    let _refresh_token = token[1];

    let claim = match validate_token(Algorithm::HS256, &jwt_access_key, &access_token) {
        Ok(token_data) => token_data.claims,
        Err(_) => {
            return Err((
                ResponseError::new(
                    "Access Token is not valid".to_owned(),
                    StatusCode::FORBIDDEN,
                    -1,
                )
                .into(),
                req,
            ))
        }
    };

    add_headers(&mut req, &claim);
    return Ok(req);
}

fn add_headers(req: &mut ServiceRequest, claim: &Claims) {
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

    HttpResponse::Ok()
        .append_header(("x-auth-email", auth_email))
        .append_header(("x-auth-role", auth_role))
        .append_header(("x-auth-session", auth_session))
        .json(json)
}
