use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use validator::Validate;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::utils::error::Error;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[sqlx(default)]
    pub created_at: DateTime<Utc>,
    #[sqlx(default)]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn create(pool: &PgPool, username: &str, password: &str) -> Result<Self, Error> {
        let hashed_password = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| Error::Internal(e.to_string()))?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING id, username, email, password, 
                      COALESCE(created_at, CURRENT_TIMESTAMP) as "created_at!: DateTime<Utc>",
                      COALESCE(updated_at, CURRENT_TIMESTAMP) as "updated_at!: DateTime<Utc>"
            "#,
            username,
            hashed_password
        )
        .fetch_one(pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn verify(pool: &PgPool, username: &str, password: &str) -> Result<Self, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password,
                   COALESCE(created_at, CURRENT_TIMESTAMP) as "created_at!: DateTime<Utc>",
                   COALESCE(updated_at, CURRENT_TIMESTAMP) as "updated_at!: DateTime<Utc>"
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::Auth("用户名或密码错误".to_string()))?;

        let valid = verify(password.as_bytes(), &user.password)
            .map_err(|e| Error::Internal(e.to_string()))?;

        if !valid {
            return Err(Error::Auth("用户名或密码错误".to_string()));
        }

        Ok(user)
    }

    pub fn generate_token(&self) -> Result<String, Error> {
        // TODO: 实现 JWT token 生成
        Ok("dummy_token".to_string())
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
} 