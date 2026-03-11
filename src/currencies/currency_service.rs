use axum::{Json, Router, http::{HeaderMap, HeaderValue, header::CACHE_CONTROL}, response::IntoResponse, routing::get};

use crate::{
    axum_state::State,
    currencies::currency::{CURRENCIES},
};

pub fn currency_service() -> axum::Router<State> {
    Router::new().route("/", get(list_currencies))
}

async fn list_currencies() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_static("max-age=86400, public"),
    );
    (headers, Json(CURRENCIES.clone()))
}
