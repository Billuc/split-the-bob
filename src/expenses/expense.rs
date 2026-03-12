use std::collections::HashMap;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SplitMethod {
    Evenly,
    Amounts { amounts: HashMap<String, f32> },
}

impl SplitMethod {
    pub fn is_amounts(&self) -> bool {
        matches!(self, SplitMethod::Amounts { .. })
    }

    pub fn amount_for_or_zero(&self, participant: &str) -> f32 {
        match self {
            SplitMethod::Evenly => 0.0,
            SplitMethod::Amounts { amounts } => *amounts.get(participant).unwrap_or(&0.0),
        }
    }
}
