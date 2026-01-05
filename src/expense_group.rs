use crate::expense::Expense;

pub struct ExpenseGroup {
    id: String,
    name: String,
    usernames: Vec<String>,
    expenses: Vec<Expense>
}
