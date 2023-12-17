use axum::Router;

pub mod about;
pub mod blogs_controller;
pub mod contact;
pub mod not_found_controller;

pub fn get_all_routes() -> Router {
    Router::new()
        .merge(blogs_controller::get_blogs_routes())
        .merge(about::get_about_routes())
        .merge(contact::get_contact_routes())
        .merge(not_found_controller::get_not_found_routes())
}
