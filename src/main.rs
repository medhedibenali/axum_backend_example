mod app_state;
mod config;
mod errors;
mod handlers;
mod models;

use app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use config::DB_CONFIG;
use handlers::{create_user, get_user, get_users, root};
use std::sync::Arc;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (client, connection) = tokio_postgres::connect(DB_CONFIG, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let app_state = Arc::new(AppState { client });

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
