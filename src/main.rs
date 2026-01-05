use axum::{ routing::{get, post}, Router, http::StatusCode, Json };
mod expense;
mod expense_group;
mod db;

// #[tokio::main]
fn main() {
    // let app = Router::new()
        // .route("/", get(async || { "Hello World" }));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:1221").await.unwrap();
    // let _ = axum::serve(listener, app).await;
    db::db();
}
