use std::env;

use axum::{routing, serve, Router};

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;
use tracing_subscriber;
mod controllers;
mod models;
mod services;
mod views;

#[derive(Clone)]
struct AppState {
    pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::default())?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:data.db")
        .await?;

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .merge(controllers::get_all_routes())
        .nest_service(
            "/static",
            routing::get_service(tower_http::services::ServeDir::new("./src/static")),
        )
        .with_state(state);

    let port = env::var("PORT").unwrap_or(String::from("3000"));
    let addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on http://{}", listener.local_addr()?);

    serve(listener, app).await?;
    Ok(())
}
