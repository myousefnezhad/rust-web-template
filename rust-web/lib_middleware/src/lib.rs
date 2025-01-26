use actix_web::{
    dev::ServiceRequest,
    http::{header, StatusCode},
    Error, HttpRequest, HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use lib_error::http::ResponseError;
use log::*;
use serde::Serialize;

// curl -v -X POST -H 'Authorization: Bearer access_token refresh_token' -H 'Content-Type: application/json'
pub async fn validator(
    mut req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
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

    debug!(
        "Middleware:\nAccess token {}\nRefresh token {}",
        &access_token, &refresh_token
    );

    if access_token == "access_token" || refresh_token == "refresh_token" {
        add_headers(&mut req, &access_token, &refresh_token);
        return Ok(req);
    }

    return Err((
        ResponseError::new(
            "Tokens are not valid!".to_owned(),
            StatusCode::BAD_REQUEST,
            -1,
        )
        .into(),
        req,
    ));
}

fn add_headers(req: &mut ServiceRequest, access_token: &str, refresh_token: &str) {
    req.headers_mut().insert(
        header::HeaderName::from_static("x-auth-access-token"),
        header::HeaderValue::from_bytes(access_token.as_bytes()).unwrap(),
    );
    req.headers_mut().insert(
        header::HeaderName::from_static("x-auth-refresh-token"),
        header::HeaderValue::from_bytes(refresh_token.as_bytes()).unwrap(),
    );
    req.headers_mut().insert(
        header::HeaderName::from_static("x-auth-user"),
        header::HeaderValue::from_bytes("tony".as_bytes()).unwrap(),
    );
}

pub fn json_response<T>(json: &T, req: &HttpRequest) -> impl Responder
where
    T: Serialize,
{
    let auth_access_token = req
        .headers()
        .get(header::HeaderName::from_static("x-auth-access-token"))
        .unwrap()
        .to_str()
        .unwrap();

    let auth_refresh_token = req
        .headers()
        .get(header::HeaderName::from_static("x-auth-refresh-token"))
        .unwrap()
        .to_str()
        .unwrap();

    let auth_user = req
        .headers()
        .get(header::HeaderName::from_static("x-auth-user"))
        .unwrap()
        .to_str()
        .unwrap();

    HttpResponse::Ok()
        .append_header(("x-auth-access-token", auth_access_token))
        .append_header(("x-auth-refresh-token", auth_refresh_token))
        .append_header(("x-auth-user", auth_user))
        .json(json)
}
