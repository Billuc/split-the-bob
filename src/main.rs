use axum::{
    Json, Router, extract,
    http::StatusCode,
    routing::{get, post},
};
use std::env;
use tower_http::services::ServeDir;

use crate::axum_state::State;
use crate::db::DB;
use crate::splits::split_service::split_service;

mod axum_state;
mod db;
mod error;
mod expenses;
mod splits;
mod balances;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "1221".to_string());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://stb.data".to_string());

    let static_files = ServeDir::new("./static");
    let db = DB::new(&db_url).await.unwrap();
    let state = State { db };

    let app = Router::new()
        .nest("/split", split_service())
        .fallback_service(static_files)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Starting server on 0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}
