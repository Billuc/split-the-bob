use axum::{
    Router, extract,
    response::{IntoResponse, Response},
    routing::post,
};
use axum_extra::extract::Form;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{axum_state::State, splits::split_repo::SplitRepo};
use crate::error::Error;
use crate::expenses::expense::{Expense, SplitMethod};
use crate::expenses::expense_repo::ExpenseRepo;
use crate::splits::split_view::get_split_view;
use crate::currencies::currency;

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
    let result = try_add_expense(&state, &form).await;

    let response = match result {
        Err(error) => {
            eprintln!("Error creating expense: {:?}", error);
            get_split_view(&state.db, form.split_id.clone(), vec![error])
                .await?
                .into_response()
        }
        Ok(()) => {
            println!("Created expense for split {}", form.split_id);
            get_split_view(&state.db, form.split_id.clone(), vec![])
                .await?
                .into_response()
        }
    };

    Ok(response)
}

async fn try_add_expense(state: &State, form: &AddExpenseForm) -> Result<(), Error> {
    let split = SplitRepo::get_by_id(&state.db, form.split_id.clone()).await?;

    let default_currency = currency::try_get_currency(&split.default_currency)?;
    let expense_currency = currency::try_get_currency(&form.currency)?;
    let default_currency_amount = currency::convert(form.amount, expense_currency, default_currency).await?;

    let new_expense = Expense {
        id: 0, // Will be auto-assigned by database
        split_id: form.split_id.clone(),
        name: form.name.clone(),
        amount: default_currency_amount,
        currency: split.default_currency.clone(),
        original_amount: form.amount,
        original_currency: form.currency.clone(),
        payed_by: form.payed_by.clone(),
        payed_for: form.payed_for.clone(),
        expense_date: system_time_from_timestamp(form.expense_date),
        split_method: SplitMethod::Evenly,
    };

    ExpenseRepo::create(&state.db, new_expense).await
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
    let result = try_update_expense(&state, &form).await;

    let response = match result {
        Err(error) => {
            eprintln!("Error creating expense: {:?}", error);
            get_split_view(&state.db, form.split_id.clone(), vec![error])
                .await?
                .into_response()
        }
        Ok(()) => {
            println!("Created expense for split {}", form.split_id);
            get_split_view(&state.db, form.split_id.clone(), vec![])
                .await?
                .into_response()
        }
    };

    Ok(response)
}

async fn try_update_expense(state: &State, form: &UpdateExpenseForm) -> Result<(), Error> {
    let split = SplitRepo::get_by_id(&state.db, form.split_id.clone()).await?;

    let default_currency = currency::try_get_currency(&split.default_currency)?;
    let expense_currency = currency::try_get_currency(&form.currency)?;
    let default_currency_amount = currency::convert(form.amount, expense_currency, default_currency).await?;

    let updated_expense = Expense {
        id: form.id,
        split_id: form.split_id.clone(),
        name: form.name.clone(),
        amount: default_currency_amount,
        currency: split.default_currency.clone(),
        original_amount: form.amount,
        original_currency: form.currency.clone(),
        payed_by: form.payed_by.clone(),
        payed_for: form.payed_for.clone(),
        expense_date: system_time_from_timestamp(form.expense_date),
        split_method: SplitMethod::Evenly,
    };

    ExpenseRepo::update(&state.db, updated_expense).await
}

fn system_time_from_timestamp(timestamp: f32) -> SystemTime {
    if timestamp < 0.0 {
        return UNIX_EPOCH;
    }
    UNIX_EPOCH + Duration::from_secs_f32(timestamp)
}
