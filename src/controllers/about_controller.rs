use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tera::Context;

use crate::{views::TEMPLATES, AppState};

async fn render_about() -> impl IntoResponse {
    let tera = TEMPLATES.clone();

    match tera.render("about/index.html", &Context::new()) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            tracing::warn!("Error rendering template: {}", err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub fn get_about_routes() -> Router<Arc<AppState>> {
    Router::new().route("/about", get(render_about))
}
