pub struct Expense {
    pub id: i64,
    pub split_id: String,
    pub name: String,
    pub amount: f32,
    pub currency: String,
    pub original_amount: f32,
    pub original_currency: String,
    pub payed_by: String,
    pub payed_for: Vec<String>,
    pub expense_date: std::time::SystemTime,
    pub split_method: SplitMethod,
}

pub enum SplitMethod {
    Evenly,
    // Amounts { amounts: HashMap<String, f32> }
}
