use askama::Template;
use axum::response::Html;

use crate::error::Error;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexView {
    errors: Vec<Error>,
}

pub async fn index(errors: Vec<Error>) -> Result<Html<String>, Error> {
    let view = IndexView { errors };
    let html = view.render()?;
    Ok(Html(html))
}
