use axum::{
    Router, extract,
    response::Response,
    routing::{get, post},
};
use axum_extra::extract::Form;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::Error;
use crate::expenses::expense::{Expense, SplitMethod};
use crate::expenses::expense_repo::ExpenseRepo;
use crate::{axum_state::State, splits::split_repo::SplitRepo};
use crate::{currencies::currency, splits::redirect_to_split};

pub fn expense_service() -> axum::Router<State> {
    Router::new()
        .route("/new", post(add_expense))
        .route("/update", post(update_expense))
        .route("/delete", get(delete_expense))
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
    split_method: String,
    #[serde(rename = "amounts_person[]", default)]
    amounts_person: Vec<String>,
    #[serde(rename = "amounts_value[]", default)]
    amounts_value: Vec<f32>,
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
            redirect_to_split(&form.split_id, vec![error])?
        }
        Ok(()) => {
            println!("Created expense for split {}", form.split_id);
            redirect_to_split(&form.split_id, vec![])?
        }
    };

    Ok(response)
}

async fn try_add_expense(state: &State, form: &AddExpenseForm) -> Result<(), Error> {
    let split = SplitRepo::get_by_id(&state.db, form.split_id.clone()).await?;

    let default_currency = currency::try_get_currency(&split.default_currency)?;
    let expense_currency = currency::try_get_currency(&form.currency)?;
    let default_currency_amount =
        currency::convert(form.amount, expense_currency, default_currency).await?;
    let split_method =
        build_split_method(form.amount, &form.payed_for, &form.split_method, &form.amounts_person, &form.amounts_value)?;
    let split_method =
        convert_split_method_amounts(split_method, expense_currency, default_currency).await?;

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
        split_method,
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
    split_method: String,
    #[serde(rename = "amounts_person[]", default)]
    amounts_person: Vec<String>,
    #[serde(rename = "amounts_value[]", default)]
    amounts_value: Vec<f32>,
    expense_date: f32,
}

pub async fn update_expense(
    extract::State(state): extract::State<State>,
    Form(form): Form<UpdateExpenseForm>,
) -> Result<Response, Error> {
    let result = try_update_expense(&state, &form).await;

    let response = match result {
        Err(error) => {
            eprintln!("Error updating expense: {:?}", error);
            redirect_to_split(&form.split_id, vec![error])?
        }
        Ok(()) => {
            println!("Updated expense for split {}", form.split_id);
            redirect_to_split(&form.split_id, vec![])?
        }
    };

    Ok(response)
}

async fn try_update_expense(state: &State, form: &UpdateExpenseForm) -> Result<(), Error> {
    let split = SplitRepo::get_by_id(&state.db, form.split_id.clone()).await?;

    let default_currency = currency::try_get_currency(&split.default_currency)?;
    let expense_currency = currency::try_get_currency(&form.currency)?;
    let default_currency_amount =
        currency::convert(form.amount, expense_currency, default_currency).await?;
    let split_method =
        build_split_method(form.amount, &form.payed_for, &form.split_method, &form.amounts_person, &form.amounts_value)?;
    let split_method =
        convert_split_method_amounts(split_method, expense_currency, default_currency).await?;

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
        split_method,
    };

    ExpenseRepo::update(&state.db, updated_expense).await
}

#[derive(serde::Deserialize)]
pub struct DeleteExpenseForm {
    id: i64,
    split_id: String,
}

pub async fn delete_expense(
    extract::State(state): extract::State<State>,
    Form(form): Form<DeleteExpenseForm>,
) -> Result<Response, Error> {
    let result = try_delete_expense(&state, &form).await;

    let response = match result {
        Err(error) => {
            eprintln!("Error deleting expense: {:?}", error);
            redirect_to_split(&form.split_id, vec![error])?
        }
        Ok(()) => {
            println!("Deleted expense for split {}", form.split_id);
            redirect_to_split(&form.split_id, vec![])?
        }
    };

    Ok(response)
}

async fn try_delete_expense(state: &State, form: &DeleteExpenseForm) -> Result<(), Error> {
    ExpenseRepo::delete(&state.db, form.id, &form.split_id).await
}

fn system_time_from_timestamp(timestamp: f32) -> SystemTime {
    if timestamp < 0.0 {
        return UNIX_EPOCH;
    }
    UNIX_EPOCH + Duration::from_secs_f32(timestamp)
}

fn build_split_method(
    expense_amount: f32,
    payed_for: &[String],
    split_method: &str,
    amounts_person: &[String],
    amounts_value: &[f32],
) -> Result<SplitMethod, Error> {
    match split_method {
        "Evenly" => Ok(SplitMethod::Evenly),
        "Amounts" => {
            let amounts = build_amounts_map(amounts_person, amounts_value)?;
            validate_amount_split(expense_amount, payed_for, &amounts)?;
            Ok(SplitMethod::Amounts { amounts })
        }
        _ => Err(Error::Validation(
            "Méthode de répartition inconnue".to_string(),
        )),
    }
}

fn build_amounts_map(
    amounts_person: &[String],
    amounts_value: &[f32],
) -> Result<HashMap<String, f32>, Error> {
    if amounts_person.len() != amounts_value.len() {
        return Err(Error::Validation(
            "Les montants par participant sont invalides".to_string(),
        ));
    }

    let mut amounts: HashMap<String, f32> = HashMap::new();
    for (person, value) in amounts_person.iter().zip(amounts_value.iter()) {
        if person.is_empty() {
            continue;
        }

        if *value < 0.0 {
            return Err(Error::Validation(
                "Les montants ne peuvent pas être négatifs".to_string(),
            ));
        }

        amounts.insert(person.clone(), *value);
    }

    Ok(amounts)
}

fn validate_amount_split(
    expense_amount: f32,
    payed_for: &[String],
    amounts: &HashMap<String, f32>,
) -> Result<(), Error> {
    let allowed_people: HashSet<&String> = payed_for.iter().collect();
    for person in amounts.keys() {
        if !allowed_people.contains(person) {
            return Err(Error::Validation(format!(
                "{} n'est pas dans la liste des participants concernés",
                person
            )));
        }
    }

    let total = amounts.values().sum::<f32>();
    if (total - expense_amount).abs() > 0.01 {
        return Err(Error::Validation(
            "La somme des montants doit être égale au montant de la dépense".to_string(),
        ));
    }

    Ok(())
}

async fn convert_split_method_amounts(
    split_method: SplitMethod,
    expense_currency: &currency::Currency,
    default_currency: &currency::Currency,
) -> Result<SplitMethod, Error> {
    match split_method {
        SplitMethod::Evenly => Ok(SplitMethod::Evenly),
        SplitMethod::Amounts { amounts } => {
            let mut converted_amounts: HashMap<String, f32> = HashMap::new();

            for (participant, participant_amount) in amounts {
                let converted_amount =
                    currency::convert(participant_amount, expense_currency, default_currency).await?;
                converted_amounts.insert(participant, converted_amount);
            }

            Ok(SplitMethod::Amounts {
                amounts: converted_amounts,
            })
        }
    }
}
