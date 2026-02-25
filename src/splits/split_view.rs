use askama::Template;
use std::time::SystemTime;
use axum::response::Html;

use crate::balances::balance::{Balance, balances_from_expenses};
use crate::currencies::currency::{CURRENCIES, Currency};
use crate::error::Error;
use crate::expenses::expense::Expense;
use crate::splits::split::Split;
use crate::db::DB;
use crate::splits::split_repo::SplitRepo;
use crate::expenses::expense_repo::ExpenseRepo;

#[derive(Template)]
#[template(path = "split_view.html")]
struct SplitView<'a> {
    split: Split,
    expenses: Vec<Expense>,
    balances: Vec<Balance>,
    errors: Vec<&'a str>,
    currencies: Vec<Currency>,
}

pub async fn get_split_view(db: &DB, id: String, errors: Vec<&str>) -> Result<Html<String>, Error> {
    let split = SplitRepo::get_by_id(db, id).await?;
    let expenses = ExpenseRepo::get_for_split(db, &split.id).await?;
    let balances = balances_from_expenses(&expenses, split.default_currency.clone());

    let template = SplitView {
        split,
        expenses,
        balances,
        errors,
        currencies: (*CURRENCIES).clone()
    };
    let view = template.render()?;
    Ok(view.into())
}