use axum::{
    Router,
    routing::get,
};
use std::env;
use tower_http::services::ServeDir;

use crate::axum_state::State;
use crate::db::DB;
use crate::expenses::expense_service::expense_service;
use crate::splits::split_service::split_service;
use crate::view::index;

mod axum_state;
mod balances;
mod db;
mod error;
mod expenses;
mod splits;
mod view;
mod keys;
mod currencies;

#[tokio::main]
async fn main() {
    loadenv::load().unwrap();
    let port = env::var("PORT").unwrap_or_else(|_| "1221".to_string());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://stb.data".to_string());

    println!("Using database URL: {}", db_url);

    let static_files = ServeDir::new("./static");
    let db = DB::new(&db_url).await.unwrap();
    let state = State { db };

    let app = Router::new()
        .route("/", get(|| index(vec![])))
        .nest("/splits", split_service())
        .nest("/expenses", expense_service())
        //         .layer(middleware::from_fn(full_page_middleware))
        .fallback_service(static_files)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Starting server on 0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}


// async fn full_page_middleware(req: Request, next: Next) -> Response {
//     let partial_response = next.run(req).await;
//
//     let response = match req.headers().get("Sec-Fetch-Dest") {
//         Some("document") => {
//             todo!();
//         }
//         _ => partial_response,
//     };
//     response
// }
