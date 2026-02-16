use axum::{ routing::{get, post}, Router, http::StatusCode, Json };
use db::{DB};
mod expense;
mod expense_group;
mod db;

#[derive(Clone)]
struct State {
    db: DB,
}

#[tokio::main]
async fn main() {
    let db = DB::new(".split-the-bob-data", "splits").unwrap();
    let state = { db };

    let app = Router::new()
        .route("/", get(async || { "Hello World" }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1221").await.unwrap();
    let _ = axum::serve(listener, app).await;
}
