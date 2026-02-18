use sqlx::{FromRow, sqlite::SqliteRow};

use crate::db::DB;
use crate::splits::split::Split;
use crate::error::Error;

#[derive(FromRow)]
struct SplitDTO {
    id: String,
    code: String,
    description: String,
    usernames: String,
    default_currency: String,
}

impl From<Split> for SplitDTO {
    fn from(split: Split) -> Self {
        SplitDTO {
            id: split.id,
            code: split.code,
            description: split.description,
            usernames: split.usernames.join(","),
            default_currency: split.default_currency,
        }
    }
}

impl Into<Split> for SplitDTO {
    fn into(self) -> Split {
        Split {
            id: self.id,
            code: self.code,
            description: self.description,
            usernames: self.usernames.split(",").map(|s| s.to_string()).collect(),
            default_currency: self.default_currency,
        }
    }
}

pub struct SplitRepo {}

impl SplitRepo {
    pub async fn get_all(db: &DB) -> Result<Vec<Split>, Error> {
        let split_dtos: Vec<SplitDTO> =
            sqlx::query_as("SELECT * FROM splits")
                .fetch_all(db.get_pool())
                .await?;

        Ok(split_dtos
            .into_iter()
            .map(|dto: SplitDTO| dto.into())
            .collect())
    }

    pub async fn get_by_id(db: &DB, id: String) -> Result<Split, Error> {
        let dto: SplitDTO =
            sqlx::query_as("SELECT * FROM splits WHERE id = ?")
                .bind(id)
                .fetch_one(db.get_pool())
                .await?;

        Ok(dto.into())
    }

    pub async fn create(db: &DB, split: Split) -> Result<String, Error> {
        let dto: SplitDTO = split.into();
        let id = Self::generate_id();

        sqlx::query("INSERT INTO splits (id, code, description, usernames, default_currency) VALUES (?, ?, ?, ?, ?)")
            .bind(id.clone())
            .bind(dto.code)
            .bind(dto.description)
            .bind(dto.usernames)
            .bind(dto.default_currency)
            .execute(db.get_pool())
            .await?;

        Ok(id)
    }

    pub async fn update(db: &DB, split: Split) -> Result<(), Error> {
        let dto: SplitDTO = split.into();
        sqlx::query("UPDATE splits SET description = ?, usernames = ? WHERE id = ?")
            .bind(dto.description)
            .bind(dto.usernames)
            .bind(dto.id)
            .execute(db.get_pool())
            .await?;

        Ok(())
    }

    pub async fn delete(db: &DB, id: String) -> Result<(), Error> {
        sqlx::query("DELETE FROM splits WHERE id = ?")
            .bind(id)
            .execute(db.get_pool())
            .await?;

        Ok(())
    }

    fn generate_id() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        format!("{}", nanos)
    }
}
