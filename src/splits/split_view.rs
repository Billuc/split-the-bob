use askama::Template;
use std::time::SystemTime;
use axum::response::Html;

use crate::balances::balance::{Balance, balances_from_expenses};
use crate::error::Error;
use crate::expenses::expense::Expense;
use crate::splits::split::Split;
use crate::db::DB;
use crate::splits::split_repo::SplitRepo;
use crate::expenses::expense_repo::ExpenseRepo;

#[derive(Template)]
#[template(path = "split_view.html")]
pub struct SplitView {
    pub split: Split,
    pub expenses: Vec<Expense>,
    pub balances: Vec<Balance>,
    pub errors: Vec<Error>,
}

pub async fn get_split_view(db: &DB, id: String, errors: Vec<Error>) -> Result<Html<String>, Error> {
    let split = SplitRepo::get_by_id(db, id).await?;
    let expenses = ExpenseRepo::get_for_split(db, &split.id).await?;
    let balances = balances_from_expenses(&expenses, split.default_currency.clone());

    let template = SplitView {
        split,
        expenses,
        balances,
        errors,
    };
    let view = template.render()?;
    Ok(view.into())
}