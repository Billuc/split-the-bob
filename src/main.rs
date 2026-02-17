use axum::{ routing::{get, post}, Router, http::StatusCode, Json };
use db::{DB};
use tower_http::services::ServeDir;

mod expense;
mod expense_group;
mod db;

#[derive(Clone)]
struct State {
    db: DB,
}

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new("./static");
    let db = DB::new("sqlite://stb.data").await.unwrap();
    let state = { db };

    let app = Router::new()
        .fallback_service(static_files)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1221").await.unwrap();
    println!("Starting server on 0.0.0.0:1221");
    axum::serve(listener, app).await.unwrap();
}

