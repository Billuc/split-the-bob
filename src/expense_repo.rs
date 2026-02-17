use crate::db::{DB, Error};
use crate::split::{Expense, SplitMethod};
use sqlx::{FromRow, sqlite::SqliteRow, types::Json};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(FromRow)]
struct ExpenseDTO {
    id: i64,
    split_id: String,
    name: String,
    amount: f32,
    currency: String,
    payed_by: String,
    payed_for: String,
    expense_date: u32,
    split_method: Json<SplitMethodDTO>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SplitMethodDTO {
    method: String,
    details: String,
}

impl From<Expense> for ExpenseDTO {
    fn from(expense: Expense) -> Self {
        ExpenseDTO {
            id: expense.id,
            split_id: expense.split_id,
            name: expense.name,
            amount: expense.amount,
            currency: expense.currency,
            payed_by: expense.payed_by,
            payed_for: expense.payed_for.join(","),
            expense_date: expense.expense_date.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO).as_secs() as u32,
            split_method: Json(expense.split_method.into()),
        }
    }
}

impl Into<Expense> for ExpenseDTO {
    fn into(self) -> Expense {
        Expense {
            id: self.id,
            split_id: self.split_id,
            name: self.name,
            amount: self.amount,
            currency: self.currency,
            payed_by: self.payed_by,
            payed_for: self.payed_for.split(",").map(|s| s.to_string()).collect(),
            expense_date: UNIX_EPOCH + Duration::from_secs(self.expense_date as u64),
            split_method: self.split_method.0.into(),
        }
    }
}

impl From<SplitMethod> for SplitMethodDTO {
    fn from(method: SplitMethod) -> Self {
        match method {
            SplitMethod::Evenly => SplitMethodDTO {
                method: "Evenly".to_string(),
                details: "".to_string(),
            },
            // SplitMethod::Amounts { amounts } => SplitMethodDTO {
            //     method: "Amounts".to_string(),
            //     details: serde_json::to_string(&amounts).unwrap(),
            // },
        }
    }
}

impl Into<SplitMethod> for SplitMethodDTO {
    fn into(self) -> SplitMethod {
        match self.method.as_str() {
            "Evenly" => SplitMethod::Evenly,
            // "Amounts" => SplitMethod::Amounts {
            //     amounts: serde_json::from_str(&self.details).unwrap(),
            // },
            _ => {
                println!("Unknown split method");
                SplitMethod::Evenly
            }
        }
    }
}

struct ExpenseRepo {}

impl ExpenseRepo {
    pub async fn get_all(db: DB) -> Result<Vec<Expense>, Error> {
        let expense_dtos: Vec<ExpenseDTO> = sqlx::query_as("SELECT * FROM expenses")
            .fetch_all(db.get_pool())
            .await?;


        Ok(expense_dtos.into_iter().map(|dto: ExpenseDTO| dto.into()).collect())
    }

    pub async fn get_for_split(db: DB, split_id: String) -> Result<Vec<Expense>, Error> {
        let expense_dtos: Vec<ExpenseDTO> = sqlx::query_as("SELECT * FROM expenses WHERE split_id = ?")
            .bind(split_id)
            .fetch_all(db.get_pool())
            .await?;

        Ok(expense_dtos.into_iter().map(|dto: ExpenseDTO| dto.into()).collect())
    }

    pub async fn create(db: DB, expense: Expense) -> Result<(), Error> {
        let dto: ExpenseDTO = expense.into();
        sqlx::query(r#"
        INSERT INTO expenses (split_id, name, amount, currency, payed_by, payed_for, expense_date, split_method) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#)
            .bind(dto.split_id)
            .bind(dto.name)
            .bind(dto.amount)
            .bind(dto.currency)
            .bind(dto.payed_by)
            .bind(dto.payed_for)
            .bind(dto.expense_date)
            .bind(dto.split_method)
            .execute(db.get_pool())
            .await?;

        Ok(())
    }

    pub async fn update(db: DB, expense: Expense) -> Result<(), Error> {
        let dto: ExpenseDTO = expense.into();
        sqlx::query(r#"
        UPDATE expenses 
        SET name = ?, 
            amount = ?,
            currency = ?,
            payed_by = ?,
            payed_for = ?,
            expense_date = ?,
            split_method = ?
        WHERE id = ?
        "#)
            .bind(dto.name)
            .bind(dto.amount)
            .bind(dto.currency)
            .bind(dto.payed_by)
            .bind(dto.payed_for)
            .bind(dto.expense_date)
            .bind(dto.split_method)
            .bind(dto.id)
            .execute(db.get_pool())
            .await?;

        Ok(())
    }

    pub async fn delete(db: DB, id: String) -> Result<(), Error> {
        sqlx::query("DELETE FROM expenses WHERE id = ?")
            .bind(id)
            .execute(db.get_pool())
            .await?;

        Ok(())
    }
}
