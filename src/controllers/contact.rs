use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tera::Context;

use crate::views::TEMPLATES;

async fn contact() -> impl IntoResponse {
    let tera = TEMPLATES.clone();
    match tera.render("contact/index.html", &Context::new()) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            println!("Error rendering template: {}", err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub fn get_contact_routes() -> Router {
    Router::new().route("/contact", get(contact))
}
