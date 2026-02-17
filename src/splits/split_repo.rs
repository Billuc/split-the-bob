use sqlx::{FromRow, sqlite::SqliteRow};

use crate::db::DB;
use crate::splits::split::Split;
use crate::error::Error;

#[derive(FromRow)]
struct SplitDTO {
    id: String,
    description: String,
    usernames: String,
}

impl From<Split> for SplitDTO {
    fn from(split: Split) -> Self {
        SplitDTO {
            id: split.id,
            description: split.description,
            usernames: split.usernames.join(","),
        }
    }
}

impl Into<Split> for SplitDTO {
    fn into(self) -> Split {
        Split {
            id: self.id,
            description: self.description,
            usernames: self.usernames.split(",").map(|s| s.to_string()).collect(),
        }
    }
}

pub struct SplitRepo {}

impl SplitRepo {
    pub async fn get_all(db: &DB) -> Result<Vec<Split>, Error> {
        let split_dtos: Vec<SplitDTO> =
            sqlx::query_as("SELECT id, description, usernames FROM splits")
                .fetch_all(db.get_pool())
                .await?;

        Ok(split_dtos
            .into_iter()
            .map(|dto: SplitDTO| dto.into())
            .collect())
    }

    pub async fn get_by_id(db: &DB, id: String) -> Result<Split, Error> {
        let dto: SplitDTO =
            sqlx::query_as("SELECT id, description, usernames FROM splits WHERE id = ?")
                .bind(id)
                .fetch_one(db.get_pool())
                .await?;

        Ok(dto.into())
    }

    pub async fn create(db: &DB, split: Split) -> Result<String, Error> {
        let dto: SplitDTO = split.into();
        let id = Self::generate_id();

        sqlx::query("INSERT INTO splits (id, description, usernames) VALUES (?, ?, ?)")
            .bind(id.clone())
            .bind(dto.description)
            .bind(dto.usernames)
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
