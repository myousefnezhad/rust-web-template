use actix_web::{post, HttpRequest, Responder, Result};
use lib_error::http::ResponseError;
use lib_middleware::json_response;
use log::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PingInfo {
    message: String,
}

// curl -v -X POST -H 'Content-Type: application/json' -H 'Authorization: Bearer A B' http://localhost:9000/auth/ping
#[post("/ping")]
pub async fn post_ping(req: HttpRequest) -> Result<impl Responder, ResponseError> {
    debug!("ping");
    let res = PingInfo {
        message: "pong".to_owned(),
    };
    Ok(json_response::<PingInfo>(&res, &req))
}
