use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum Error {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    #[error("内部错误: {0}")]
    Internal(String),
    #[error("认证错误: {0}")]
    Auth(String),
    #[error("验证错误: {0}")]
    Validation(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Database(_) => HttpResponse::InternalServerError().json(self.to_string()),
            Error::Validation(msg) => HttpResponse::BadRequest().json(msg),
            Error::Auth(msg) => HttpResponse::Unauthorized().json(msg),
            Error::Internal(msg) => HttpResponse::InternalServerError().json(msg),
        }
    }
} 