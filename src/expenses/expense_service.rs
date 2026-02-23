use axum::{
    Router, extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use axum_extra::extract::Form;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::axum_state::State;
use crate::error::Error;
use crate::expenses::expense::{Expense, SplitMethod};
use crate::expenses::expense_repo::ExpenseRepo;

pub fn expense_service() -> axum::Router<State> {
    Router::new()
        .route("/new", post(add_expense))
        .route("/update", post(update_expense))
}

#[derive(serde::Deserialize)]
pub struct AddExpenseForm {
    split_id: String,
    name: String,
    amount: f32,
    currency: String,
    payed_by: String,
    #[serde(rename = "payed_for[]")]
    payed_for: Vec<String>,
    expense_date: f32,
}

pub async fn add_expense(
    extract::State(state): extract::State<State>,
    Form(form): Form<AddExpenseForm>,
) -> Result<Response, Error> {
    let new_expense = Expense {
        id: 0, // Will be auto-assigned by database
        split_id: form.split_id.clone(),
        name: form.name.clone(),
        amount: form.amount,
        currency: form.currency.clone(),
        original_amount: form.amount,
        original_currency: form.currency.clone(),
        payed_by: form.payed_by.clone(),
        payed_for: form.payed_for.clone(),
        expense_date: system_time_from_timestamp(form.expense_date),
        split_method: SplitMethod::Evenly,
    };

    let response = match ExpenseRepo::create(&state.db, new_expense).await {
        Err(error) => {
            eprintln!("Error creating expense: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
        Ok(()) => {
            println!("Created expense for split {}", form.split_id);
            (
                StatusCode::SEE_OTHER,
                [("Location", format!("/splits?split_id={}", form.split_id))],
            )
                .into_response()
        }
    };

    Ok(response)
}

#[derive(serde::Deserialize)]
pub struct UpdateExpenseForm {
    id: i64,
    split_id: String,
    name: String,
    amount: f32,
    currency: String,
    payed_by: String,
    #[serde(rename = "payed_for[]")]
    payed_for: Vec<String>,
    expense_date: f32,
}

pub async fn update_expense(
    extract::State(state): extract::State<State>,
    Form(form): Form<UpdateExpenseForm>,
) -> Result<Response, Error> {
    let updated_expense = Expense {
        id: form.id,
        split_id: form.split_id.clone(),
        name: form.name.clone(),
        amount: form.amount,
        currency: form.currency.clone(),
        original_amount: form.amount,
        original_currency: form.currency.clone(),
        payed_by: form.payed_by.clone(),
        payed_for: form.payed_for.clone(),
        expense_date: system_time_from_timestamp(form.expense_date),
        split_method: SplitMethod::Evenly,
    };

    let response = match ExpenseRepo::update(&state.db, updated_expense).await {
        Err(error) => {
            eprintln!("Error updating expense: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
        Ok(()) => {
            println!("Updated expense {} for split {}", form.id, form.split_id);
            (
                StatusCode::SEE_OTHER,
                [("Location", format!("/splits?split_id={}", form.split_id))],
            )
                .into_response()
        }
    };

    Ok(response)
}

fn system_time_from_timestamp(timestamp: f32) -> SystemTime {
    if timestamp < 0.0 {
        return UNIX_EPOCH;
    }
    UNIX_EPOCH + Duration::from_secs_f32(timestamp)
}
