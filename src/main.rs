use axum::http::{HeaderValue, header::CACHE_CONTROL};
use axum::{Router, routing::get};
use tower_http::set_header::SetResponseHeaderLayer;
use std::env;
use tower::{ServiceBuilder};
use tower_http::services::ServeDir;

use crate::axum_state::State;
use crate::currencies::currency_service::currency_service;
use crate::db::DB;
use crate::expenses::expense_service::expense_service;
use crate::splits::split_service::split_service;
use crate::view::index;

mod axum_state;
mod balances;
mod currencies;
mod db;
mod error;
mod expenses;
mod keys;
mod splits;
mod view;

#[tokio::main]
async fn main() {
    loadenv::load().unwrap();
    let port = env::var("PORT").unwrap_or_else(|_| "1221".to_string());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://stb.data".to_string());

    println!("Using database URL: {}", db_url);

    let static_files_service = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("max-age=86400, public"),
        ))
        .service(ServeDir::new("./static"));
    let db = DB::new(&db_url).await.unwrap();
    let state = State { db };

    let app = Router::new()
        .route("/", get(|| index(vec![])))
        .nest("/splits", split_service())
        .nest("/expenses", expense_service())
        .nest("/currencies", currency_service())
        //         .layer(middleware::from_fn(full_page_middleware))
        .fallback_service(static_files_service)
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
