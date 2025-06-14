use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Database error occurred"
                }))
            }
            AppError::Auth(msg) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": msg
                }))
            }
            AppError::Validation(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": msg
                }))
            }
            AppError::Internal(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": msg
                }))
            }
        }
    }
} 