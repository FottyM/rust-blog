use axum::Router;

pub mod blogs;

pub fn get_all_routes() -> Router {
    Router::new().merge(blogs::get_blogs_routes())
}
