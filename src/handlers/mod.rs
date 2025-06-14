use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use utoipa::OpenApi;

pub mod auth;
pub mod users;

/// API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register,
        auth::login,
        users::me,
        health_check,
    ),
    components(
        schemas(
            auth::RegisterRequest,
            auth::LoginRequest,
            auth::TokenResponse,
            crate::models::user::UserResponse
        )
    ),
    tags(
        (name = "auth", description = "认证相关的API"),
        (name = "users", description = "用户相关的API"),
        (name = "system", description = "系统相关的API"),
    )
)]
pub struct ApiDoc;

/// 根路径处理函数
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": "Orbiter API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "A Rust-based Web API service",
        "documentation": "/swagger-ui/",
        "health_check": "/api/health"
    }))
}

/// API根路径处理函数
#[get("")]
pub async fn api_index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "available",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "auth": {
                "register": "/api/auth/register",
                "login": "/api/auth/login"
            },
            "users": {
                "me": "/api/users/me"
            },
            "system": {
                "health": "/api/health"
            }
        }
    }))
}

/// 健康检查接口
#[utoipa::path(
    get,
    path = "/api/health",
    tag = "system",
    responses(
        (status = 200, description = "系统健康状态", body = inline(serde_json::Value))
    )
)]
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "services": {
            "api": "up",
            "database": "up"
        }
    }))
} 