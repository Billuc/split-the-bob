use std::time::SystemTime;
use std::fmt::Display;

use crate::balances::balance::Balance;
use crate::error::Error;
use crate::expenses::expense::Expense;
use crate::splits::split::Split;
use askama::Template;
use askama::filters::HtmlSafe;

#[derive(Template)]
#[template(path = "split_view.html")]
pub struct SplitView {
    pub split: Split,
    pub expenses: Vec<Expense>,
    pub balances: Vec<Balance>,
}

#[derive(Template)]
#[template(path = "new_split.html")]
pub struct NewSplitView {
    pub errors: Vec<Error>,
}