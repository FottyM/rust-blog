use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};

use crate::views::TEMPLATES;

async fn get_not_found() -> impl IntoResponse {
    let tera = TEMPLATES.clone();

    match tera.render("404.html", &tera::Context::new()) {
        Ok(rendered) => {
            tracing::warn!("Not found");
            Html(rendered).into_response()
        }
        Err(err) => {
            println!("Error rendering template: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn get_not_found_routes() -> Router {
    Router::new().fallback(get_not_found)
}
