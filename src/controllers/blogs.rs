use crate::{
    models::blog::{Blog, Status},
    views::TEMPLATES,
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tera::Context;

use serde::{Deserialize, Serialize};

async fn get_blogs() -> impl IntoResponse {
    let tera = TEMPLATES.clone();

    let mut context = Context::new();

    let posts: Vec<Blog> = (1..=2)
        .into_iter()
        .map(|i| {
            Blog::new(
                i,
                "Blog one".to_string(),
                "lorem ipsum".to_string(),
                "2022-01-01".to_string(),
                "2022-01-01".to_string(),
                1,
                None,
                Status::default(),
            )
        })
        .collect();

    context.insert("posts", &posts);

    match tera.render("blogs/index.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            println!("Error rendering template: {}", err);
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

async fn get_blog(Path(id): Path<u32>) -> impl IntoResponse {
    // format!("Blog {}", id).into_response()
    let tera = TEMPLATES.clone();
    let mut context = Context::new();

    let post: Post = Post {
        blog: Blog::new(
            id,
            "Blog one".to_string(),
            "lorem ipsum".repeat(1_000).to_string(),
            "2022-01-01".to_string(),
            "2022-01-01".to_string(),
            1,
            None,
            Status::Published,
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

    match tera.render("blogs/show.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            println!("Error rendering template: {}", err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub fn get_blogs_routes() -> Router {
    Router::new()
        .route("/", get(get_blogs))
        .route("/blogs", get(get_blogs))
        .route("/blogs/:id", get(get_blog))
}
