use axum::{
    Json, Router, extract,
    http::StatusCode,
    routing::{get, post},
};
use db::DB;
use std::env;
use tower_http::services::ServeDir;

mod db;
mod split;
mod split_repo;
mod expense_repo;

#[derive(Clone)]
struct State {
    db: DB,
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "1221".to_string());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://stb.data".to_string());

    let static_files = ServeDir::new("./static");
    let db = DB::new(&db_url).await.unwrap();
    let state = { db };

    let app = Router::new()
        .fallback_service(static_files)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Starting server on 0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize)]
struct SplitQuery {
    split_name: String,
    split_code: String,
}

async fn show_split(
    extract::State(state): extract::State<State>,
    extract::Query(query): extract::Query<SplitQuery>,
) {
}
