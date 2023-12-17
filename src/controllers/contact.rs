use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tera::Context;

use crate::{
    models::contact_message::CreateContactMessageDto, services::contact_service, views::TEMPLATES,
    AppState,
};

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

// async fn create_contact_message(// State(state): State<Arc<AppState>>,
//     // Json(payload): Json<CreateContactMessageDto>,
// ) -> impl IntoResponse {
//     let response = contact_service::create_contact_message(payload, &state.pool).await;

//     match response {
//         Ok(_) => StatusCode::OK.into_response(),
//         Err(err) => {
//             println!("Error creating contact: {}", err);
//             StatusCode::UNPROCESSABLE_ENTITY.into_response()
//         }
//     }
// }

pub fn get_contact_routes() -> Router {
    Router::new().route("/contact", get(contact))
    // .route("/contact", post(create_contact_message))
}
