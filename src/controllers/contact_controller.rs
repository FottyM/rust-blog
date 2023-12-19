use regex::Regex;
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};

use tera::Context;

use crate::{
    models::contact_message::CreateContactMessageDto, services::contact_service, views::TEMPLATES,
    AppState,
};

async fn render_contact() -> impl IntoResponse {
    let tera = TEMPLATES.clone();
    match tera.render("contact/index.html", &Context::new()) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => {
            tracing::warn!("Error rendering template: {}", err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

async fn create_contact_message(
    State(state): State<Arc<AppState>>,
    Form(dto): Form<CreateContactMessageDto>,
) -> impl IntoResponse {
    if dto.name.is_empty() || dto.email.is_empty() || dto.message.is_empty() {
        tracing::warn!("Invalid contact message: {:?}", dto);
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
        .expect("Invalid email regex");

    if !email_regex.is_match(&dto.email) {
        tracing::warn!("Invalid email address: {}", dto.email);
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    tracing::info!("Creating contact message: {:?}", dto);

    let res = contact_service::create_contact_message(dto, &state.pool)
        .await
        .map_err(|err| {
            tracing::error!("Error creating contact message: {}", err);
            err
        });

    tracing::info!("Query result: {:?}", res);
    StatusCode::CREATED.into_response()
}

pub fn get_contact_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/contact", get(render_contact))
        .route("/submit-contact-form", post(create_contact_message))
}
