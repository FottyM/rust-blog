use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{views::TEMPLATES, AppState};

pub async fn render_admin() -> impl IntoResponse {
    let tera = TEMPLATES.clone();

    match tera.render("admin/index.html", &tera::Context::new()) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            tracing::error!("{:?}", err);
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

pub fn get_admin_routes() -> Router<Arc<AppState>> {
    Router::new().route("/admin", get(render_admin))
}
