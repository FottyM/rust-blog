use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Row};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Blog {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub author_id: u32,
    pub category_id: Option<u32>,
    pub status: String,
}

pub struct CreateBlogDto {
    pub title: String,
    pub content: String,
    pub author_id: u32,
    pub category_id: Option<u32>,
    pub status: String,
}

impl CreateBlogDto {
    pub fn new(title: String, content: String, author_id: u32, category_id: Option<u32>) -> Self {
        Self {
            title,
            content,
            author_id,
            category_id,
            status: String::from("draft"),
        }
    }
}

impl Blog {
    pub fn new(
        id: u32,
        title: String,
        content: String,
        published_at: Option<String>,
        created_at: String,
        updated_at: String,
        author_id: u32,
        category_id: Option<u32>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            published_at,
            created_at,
            updated_at,
            author_id,
            category_id,
            status: String::from("draft"),
        }
    }

    pub fn from_row(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            author_id: row.get("author_id"),
            category_id: row.get("category_id"),
            status: row.get("status"),
        }
    }
}
