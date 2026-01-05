use std::{self, collections::HashMap};

pub struct Expense {
    id: String,
    name: String,
    amount: f32,
    currency: String,
    payed_by: String,
    expense_date: std::time::SystemTime,
    split: ExpenseSplit,
}

pub enum ExpenseSplit {
    Evenly,
    // Amounts { amounts: HashMap<String, f32> }
}
