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
}

impl Expense {
    pub fn amount_for(&self, participant: &str) -> f32 {
        if !self.payed_for.contains(&participant.to_string()) {
            return 0.0;
        }

        match self.split_method {
            SplitMethod::Evenly => self.amount / self.payed_for.len() as f32,
            SplitMethod::Amounts { ref amounts } => *amounts.get(participant).unwrap_or(&0.0),
        }
    }
}
