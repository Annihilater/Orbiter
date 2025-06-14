use crate::config::Config;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // user id
    pub exp: i64, // expiration time
}

impl Claims {
    #[allow(dead_code)]
    pub fn new(user_id: i32) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        Claims {
            sub: user_id,
            exp: expiration,
        }
    }
}

#[allow(dead_code)]
pub fn create_token(user_id: i32, config: &Config) -> Result<String, Error> {
    let claims = Claims::new(user_id);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, config: &Config) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
} 