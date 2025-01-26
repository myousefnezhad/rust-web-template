use actix_web::error;
use actix_web::http::StatusCode;
use actix_web::{body::BoxBody, HttpResponse};
use bcrypt::BcryptError;
use deadpool_redis::redis::RedisError;
use derive_more::{Display, Error};
use jsonwebtoken::errors::Error as JWTError;
use log::*;
use rsa::Error as RSAError;
use serde::Serialize;
use sqlx::Error as SQLError;
use std::default::Default;
use std::io::Error as IOError;

pub static SYSTEM_ERROR_CODE: i64 = -1000;
static SYSTEM_ERROR_CODE_DB: i64 = -1001;
static SYSTEM_ERROR_CODE_IO: i64 = -1002;
static SYSTEM_ERROR_CODE_CRYPTO: i64 = -1003;

#[derive(Debug, Display, Error, Default)]
#[display(
    fmt = "{{\"message\": \"{}\", \"status\": {}, \"code\": {}}}",
    message,
    status,
    code
)]
pub struct ResponseError {
    pub message: String,
    pub status: StatusCode,
    pub code: i64,
}

#[derive(Debug, Serialize)]
pub struct ResponseErrorJson<'a> {
    pub message: &'a str,
    pub status: String,
    pub code: &'a i64,
}

impl ResponseError {
    pub fn new(message: String, status: StatusCode, code: i64) -> Self {
        Self {
            message,
            status,
            code,
        }
    }
}

impl From<SQLError> for ResponseError {
    fn from(value: SQLError) -> Self {
        Self {
            message: format!("{:?}", value),
            status: StatusCode::BAD_REQUEST,
            code: SYSTEM_ERROR_CODE_DB,
        }
    }
}

impl From<RedisError> for ResponseError {
    fn from(value: RedisError) -> Self {
        Self {
            message: format!("{:?}", value),
            status: StatusCode::BAD_REQUEST,
            code: SYSTEM_ERROR_CODE_DB,
        }
    }
}

impl From<IOError> for ResponseError {
    fn from(value: IOError) -> Self {
        Self {
            message: format!("{:?}", value),
            status: StatusCode::BAD_REQUEST,
            code: SYSTEM_ERROR_CODE_IO,
        }
    }
}

impl From<JWTError> for ResponseError {
    fn from(value: JWTError) -> Self {
        Self {
            message: format!("{:?}", value),
            status: StatusCode::BAD_REQUEST,
            code: SYSTEM_ERROR_CODE_CRYPTO,
        }
    }
}

impl From<RSAError> for ResponseError {
    fn from(value: RSAError) -> Self {
        Self {
            message: format!("{:?}", value),
            status: StatusCode::BAD_REQUEST,
            code: SYSTEM_ERROR_CODE_CRYPTO,
        }
    }
}

impl From<BcryptError> for ResponseError {
    fn from(value: BcryptError) -> Self {
        Self {
            message: format!("{:?}", value),
            status: StatusCode::BAD_REQUEST,
            code: SYSTEM_ERROR_CODE_CRYPTO,
        }
    }
}

impl error::ResponseError for ResponseError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        warn!(
            "Response Error ({}): {} {}",
            &self.code,
            &self.message,
            self.status.to_string(),
        );
        HttpResponse::build(self.status_code())
            .insert_header(("Content-Type", "application/json"))
            .json(&ResponseErrorJson {
                message: &self.message,
                status: self.status.to_string(),
                code: &self.code,
            })
    }
}
