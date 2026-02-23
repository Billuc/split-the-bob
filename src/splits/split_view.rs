use askama::Template;
use std::time::SystemTime;

use crate::balances::balance::Balance;
use crate::error::Error;
use crate::expenses::expense::Expense;
use crate::splits::split::Split;

#[derive(Template)]
#[template(path = "split_view.html")]
pub struct SplitView {
    pub split: Split,
    pub expenses: Vec<Expense>,
    pub balances: Vec<Balance>,
    pub errors: Vec<Error>,
}

#[derive(Template)]
#[template(path = "new_split.html")]
pub struct NewSplitView {
    pub errors: Vec<Error>,
}
