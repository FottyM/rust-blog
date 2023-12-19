use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};

use crate::{views::TEMPLATES, AppState};

async fn render_not_found(request: axum::http::Request<axum::body::Body>) -> impl IntoResponse {
    let tera = TEMPLATES.clone();

    match tera.render("404.html", &tera::Context::new()) {
        Ok(rendered) => {
            tracing::warn!("Request: {:?}{:?}", request.method(), request.uri());
            Html(rendered).into_response()
        }
        Err(err) => {
            tracing::error!("Error rendering template: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn get_not_found_routes() -> Router<Arc<AppState>> {
    Router::new().fallback(render_not_found)
}
