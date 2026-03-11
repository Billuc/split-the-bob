use askama::Template;
use axum::response::Html;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::balances::balance::{Balance, balances_from_expenses};
use crate::db::DB;
use crate::error::Error;
use crate::expenses::expense::Expense;
use crate::expenses::expense_repo::ExpenseRepo;
use crate::splits::split::Split;
use crate::splits::split_repo::SplitRepo;

#[derive(Template)]
#[template(path = "split_view.html")]
struct SplitView<'a> {
    errors: Vec<&'a str>,
    split: Split,
    expenses: Vec<Expense>,
    balances: Vec<Balance>,
    individual_balances: HashMap<String, f32>,
}

pub async fn get_split_view(db: &DB, id: String, errors: Vec<&str>) -> Result<Html<String>, Error> {
    let split = SplitRepo::get_by_id(db, id).await?;
    let expenses = ExpenseRepo::get_for_split(db, &split.id).await?;
    let (balances, individual_balances) = balances_from_expenses(&expenses, split.default_currency.clone());

    let template = SplitView {
        errors,
        split,
        expenses,
        balances,
        individual_balances,
    };
    let view = template.render()?;
    Ok(view.into())
}

#[derive(Template)]
#[template(path = "split_details.html")]
pub enum SplitDetails {
    #[template(block = "success")]
    Success { split: Split },
    #[template(block = "failure")]
    Failure { split_id: String },
}
