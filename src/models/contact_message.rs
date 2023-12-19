use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
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

impl Default for CreateContactMessageDto {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            message: String::new(),
        }
    }
}
