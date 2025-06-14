use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::models::user::User;
use crate::utils::error::Error;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub token: String,
}

/// 用户注册
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    tag = "auth",
    responses(
        (status = 201, description = "注册成功", body = TokenResponse),
        (status = 400, description = "请求参数错误"),
        (status = 409, description = "用户名已存在")
    )
)]
pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> Result<impl Responder, Error> {
    let user = User::create(pool.get_ref(), &req.username, &req.password).await?;
    let token = user.generate_token()?;
    
    Ok(HttpResponse::Created().json(TokenResponse { token }))
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    tag = "auth",
    responses(
        (status = 200, description = "登录成功", body = TokenResponse),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "用户名或密码错误")
    )
)]
pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> Result<impl Responder, Error> {
    let user = User::verify(pool.get_ref(), &req.username, &req.password).await?;
    let token = user.generate_token()?;
    
    Ok(HttpResponse::Ok().json(TokenResponse { token }))
} 