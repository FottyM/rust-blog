use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactMessage {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub message: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContactMessageDto {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl CreateContactMessageDto {
    pub fn new(name: String, email: String, message: String) -> Self {
        Self {
            name,
            email,
            message,
        }
    }
}
