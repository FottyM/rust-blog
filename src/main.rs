use std::env;

use axum::{serve, Router};

mod controllers;
mod models;
mod views;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().merge(controllers::get_all_routes());

    let port = env::var("PORT").unwrap_or(String::from("3000"));
    let addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Listening on http://{}", listener.local_addr()?);

    serve(listener, app).await.unwrap();

    Ok(())
}
