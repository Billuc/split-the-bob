use axum::{Router, middleware::{self, Next}, response::{Response, Html}, extract::Request, routing::get};
use std::env;
use tower_http::services::ServeDir;
use askama::Template;

use crate::axum_state::State;
use crate::db::DB;
use crate::error::Error;
use crate::splits::split_service::split_service;

mod axum_state;
mod balances;
mod db;
mod error;
mod expenses;
mod splits;
mod view;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "1221".to_string());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://stb.data".to_string());

    let static_files = ServeDir::new("./static");
    let db = DB::new(&db_url).await.unwrap();
    let state = State { db };

    let app = Router::new()
        .route("/", get(index))
        .nest("/split", split_service())
//         .layer(middleware::from_fn(full_page_middleware))
        .fallback_service(static_files)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Starting server on 0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Result<Html<String>, Error> {
    let view = view::IndexView;
    let html = view.render()?;
    Ok(Html(html))
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
