use axum::Router;

pub mod about;
pub mod blogs;
pub mod contact;

pub fn get_all_routes() -> Router {
    Router::new()
        .merge(blogs::get_blogs_routes())
        .merge(about::get_about_routes())
        .merge(contact::get_contact_routes())
}
