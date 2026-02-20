use askama::Template;
use axum::{
    Router, extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::Form;

use crate::{axum_state::State, balances::balance::balances_from_expenses};
use crate::db::DB;
use crate::error::Error;
use crate::expenses::expense_repo::ExpenseRepo;
use crate::splits::split::Split;
use crate::splits::split_repo::SplitRepo;
use crate::splits::split_view;

pub fn split_service() -> axum::Router<State> {
    Router::new()
        .route("/", get(show_split))
        .route("/new", get(new_split_form))
        .route("/new", post(create_split))
}

#[derive(serde::Deserialize)]
struct SplitQuery {
    split_id: String,
}

async fn show_split(
    extract::State(state): extract::State<State>,
    extract::Query(query): extract::Query<SplitQuery>,
) -> Result<impl IntoResponse, Error> {
    get_split_view(&state.db, query.split_id).await
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
) -> Result<Response, Error> {
    let new_split = Split {
        id: String::new(),
        description: form.description.clone(),
        usernames: form.participants.clone(),
        default_currency: form.default_currency.clone(),
    };

    let response = match SplitRepo::create(&state.db, new_split).await {
        Err(error) => {
            eprintln!("Error creating split: {:?}", error);
            new_split_view(vec![error]).into_response()
        }
        Ok(id) => {
            println!("Created split with id {}", id);
            (
                StatusCode::SEE_OTHER,
                [("Location", format!("/splits?split_id={id}"))],
            )
                .into_response()
        }
    };

    Ok(response)
}

pub async fn new_split_form() -> Result<impl IntoResponse, Error> {
    new_split_view(vec![])
}

async fn get_split_view(db: &DB, id: String) -> Result<Html<String>, Error> {
    let split = SplitRepo::get_by_id(db, id).await?;
    let expenses = ExpenseRepo::get_for_split(db, &split.id).await?;
    let balances = balances_from_expenses(&expenses, split.default_currency.clone());

    let template = split_view::SplitView {
        split,
        expenses,
        balances,
    };
    let view = template.render()?;
    Ok(view.into())
}

fn new_split_view(errors: Vec<Error>) -> Result<Html<String>, Error> {
    let template = split_view::NewSplitView { errors };
    let view = template.render()?;
    Ok(view.into())
}
