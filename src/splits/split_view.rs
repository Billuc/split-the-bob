use askama::Template;
use crate::expenses::expense::Expense;
use crate::splits::split::Split;
use crate::balances::balance::Balance;

#[derive(Template)]
#[template(path = "split_view.html")]
pub struct SplitView {
    pub split: Split,
    pub expenses: Vec<Expense>,
    pub balances: Vec<Balance>,
}
