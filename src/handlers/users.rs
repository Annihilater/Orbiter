use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use validator::Validate;
use utoipa::OpenApi;

use crate::{
    config::Config,
    models::{CreateUser, LoginUser, User, UserResponse, LoginResponse},
    utils::{create_token, error::Error, Claims},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        register,
        login,
        me,
    ),
    components(
        schemas(CreateUser, LoginUser, UserResponse, LoginResponse)
    ),
    tags(
        (name = "users", description = "User management endpoints.")
    )
)]
pub struct ApiDoc;

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<impl Responder, Error> {
    user_data.validate().map_err(|e| Error::Validation(e.to_string()))?;

    let hashed_password = hash(user_data.password.as_bytes(), DEFAULT_COST)
        .map_err(|e| Error::Internal(e.to_string()))?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password, 
                  COALESCE(created_at, CURRENT_TIMESTAMP) as "created_at!",
                  COALESCE(updated_at, CURRENT_TIMESTAMP) as "updated_at!"
        "#,
        user_data.username,
        user_data.email,
        hashed_password
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(Error::Database)?;

    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginUser,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    credentials: web::Json<LoginUser>,
) -> Result<impl Responder, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password,
               COALESCE(created_at, CURRENT_TIMESTAMP) as "created_at!",
               COALESCE(updated_at, CURRENT_TIMESTAMP) as "updated_at!"
        FROM users
        WHERE username = $1
        "#,
        credentials.username
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(Error::Database)?
    .ok_or_else(|| Error::Auth("用户名或密码错误".to_string()))?;

    let valid = verify(credentials.password.as_bytes(), &user.password)
        .map_err(|e| Error::Internal(e.to_string()))?;

    if !valid {
        return Err(Error::Auth("用户名或密码错误".to_string()));
    }

    let token = create_token(user.id, &config)
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user: UserResponse::from(user),
    }))
}

/// Get current user info
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "User info retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "users"
)]
pub async fn me(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
) -> Result<impl Responder, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password,
               COALESCE(created_at, CURRENT_TIMESTAMP) as "created_at!",
               COALESCE(updated_at, CURRENT_TIMESTAMP) as "updated_at!"
        FROM users
        WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(Error::Database)?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
} 