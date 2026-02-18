use axum::{response::IntoResponse, Router, extract, http::StatusCode, routing::{get, post}};
use axum_extra::extract::Form;
use askama::Template;

use crate::axum_state::State;
use crate::error::Error;
use crate::splits::split::Split;
use crate::splits::split_repo::SplitRepo;
use crate::expenses::expense_repo::ExpenseRepo;
use crate::splits::split_view;

pub fn split_service() -> axum::Router<State> {
    Router::new()
        .route("/", post(create_split))
        .route("/", get(show_split))
}

#[derive(serde::Deserialize)]
struct SplitQuery {
    split_id: String,
}

async fn show_split(
    extract::State(state): extract::State<State>,
    extract::Query(query): extract::Query<SplitQuery>,
) -> Result<impl IntoResponse, Error> {
    let split = SplitRepo::get_by_id(&state.db, query.split_id).await?;
    let expenses = ExpenseRepo::get_for_split(&state.db, &split.id).await?;
    let balances = vec![]; // TODO: calculate balances

    let template = split_view::SplitView {
        split,
        expenses,
        balances,
    };
    let view = template.render()?;
    Ok(view)
}

#[derive(serde::Deserialize)]
struct CreateSplitForm {
    description: String,
    #[serde(rename = "participants[]")]
    participants: Vec<String>,
    default_currency: String,
}

pub async fn create_split(
    extract::State(state): extract::State<State>,
    Form(form): Form<CreateSplitForm>,
) -> Result<impl IntoResponse, Error> {
    let new_split = Split {
        id: String::new(),
        description: form.description.clone(),
        usernames: form.participants.clone(),
        default_currency: form.default_currency.clone(),
    };

    let id = SplitRepo::create(&state.db, new_split).await?;

    println!("Created split with id {}", id);
    Ok((StatusCode::SEE_OTHER, [("Location", format!("/split?split_id={id}"))]))
}
