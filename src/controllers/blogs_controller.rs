use std::sync::Arc;

use crate::{models::blog::Blog, views::TEMPLATES, AppState};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tera::Context;

use serde::{Deserialize, Serialize};

async fn render_blogs() -> impl IntoResponse {
    let tera = TEMPLATES.clone();

    let mut context = Context::new();

    let posts: Vec<Blog> = (1..=2)
        .into_iter()
        .map(|i| {
            Blog::new(
                i,
                "Blog one".to_string(),
                "lorem ipsum".to_string(),
                Some("2022-01-01".to_string()),
                "2022-01-01".to_string(),
                "2022-01-01".to_string(),
                1,
                None,
            )
        })
        .collect();

    context.insert("posts", &posts);

    match tera.render("blogs/index.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            tracing::warn!("Error rendering template: {}", err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

// TODO: This is dummy struct for testing
#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    author: String,
    content: String,
    created_at: String,
    updated_at: String,
    publish_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    blog: Blog,
    comments: Vec<Comment>,
    tags: Vec<String>,
}

async fn render_blog(Path(id): Path<u32>) -> impl IntoResponse {
    let mut tera = TEMPLATES.clone();
    let mut context = tera::Context::new();

    let post: Post = Post {
        blog: Blog::new(
            id,
            "Blog one".to_string(),
            "lorem ipsum".repeat(1_000).to_string(),
            Some("2022-01-01".to_string()),
            "2022-01-01".to_string(),
            "2022-01-01".to_string(),
            1,
            Some(1),
        ),
        comments: vec![Comment {
            author: "John Doe".to_string(),
            content: "lorem ipsum".repeat(100).to_string(),
            created_at: "2022-01-01".to_string(),
            updated_at: "2022-01-01".to_string(),
            publish_date: "2022-01-01".to_string(),
        }],
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };

    context.insert("post", &post);

    tera.full_reload().expect("Failed to reload templates");

    match tera.render("blogs/show.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            tracing::warn!("Error rendering template: {}", err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub fn get_blogs_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(render_blogs))
        .route("/blogs", get(render_blogs))
        .route("/blogs/:id", get(render_blog))
}
