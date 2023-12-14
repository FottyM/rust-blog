use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    id: u32,
    title: String,
    content: String,
    publish_date: String,
    update_date: String,
    author_id: u32,
    category_id: Option<u32>,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Draft,
    Pending,
    Published,
    Archived,
}

impl Default for Status {
    fn default() -> Self {
        Status::Draft
    }
}

impl Blog {
    pub fn new(
        id: u32,
        title: String,
        content: String,
        publish_date: String,
        update_date: String,
        author_id: u32,
        category_id: Option<u32>,
        status: Status,
    ) -> Blog {
        Blog {
            id,
            title,
            content,
            publish_date,
            update_date,
            author_id,
            category_id,
            status,
        }
    }
}
