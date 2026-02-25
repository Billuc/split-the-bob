use askama::filters::urlencode;
use axum::http::{StatusCode, header};
use axum::response::Response;
use axum::response::IntoResponse;

use crate::error::Error;

pub(crate) mod split;
pub(crate) mod split_repo;
pub(crate) mod split_service;
pub(crate) mod split_view;

/// This function is useful when calling API endpoints.
/// Since we are using htmz and reloading the whole page, we want to have the split page's url instead of the action's url.
/// This is why we are using a redirection.
pub fn redirect_to_split(split_id: &str, errors: Vec<Error>) -> Result<Response, Error> {
    let mut url = String::from("/splits?split_id=");
    let id = urlencode(split_id)?;
    url.push_str(&id.to_string());

    if !errors.is_empty() {
        url.push_str("&errors=");
        let errs = urlencode(
            errors.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n")
        )?;
        url.push_str(&errs.to_string());
    }

    Ok((StatusCode::SEE_OTHER, [(header::LOCATION, url)]).into_response())
}
