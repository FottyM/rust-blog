use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod about_controller;
pub mod blogs_controller;
pub mod contact_controller;
pub mod not_found_controller;

pub fn get_all_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(blogs_controller::get_blogs_routes())
        .merge(about_controller::get_about_routes())
        .merge(contact_controller::get_contact_routes())
        .merge(not_found_controller::get_not_found_routes())
}
