use askama::Template;
use axum::response::Html;

use crate::{
    currencies::currency::{CURRENCIES, Currency},
    error::Error,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexView {
    errors: Vec<Error>,
    currencies: Vec<Currency>,
}

pub async fn index(errors: Vec<Error>) -> Result<Html<String>, Error> {
    let view = IndexView {
        errors,
        currencies: (*CURRENCIES).clone(),
    };
    let html = view.render()?;
    Ok(Html(html))
}
