use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub email: String,
    pub role: String,
}

impl CreateUserDto {
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        password_hash: String,
        email: String,
    ) -> CreateUserDto {
        CreateUserDto {
            username,
            first_name,
            last_name,
            password_hash,
            email,
            role: String::from("user"),
        }
    }
}
