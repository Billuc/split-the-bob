use sqlx::FromRow;

use crate::db::DB;
use crate::error::Error;
use crate::splits::split::Split;

#[derive(FromRow)]
struct SplitDTO {
    id: String,
    description: String,
    participants: String,
    default_currency: String,
}

impl From<Split> for SplitDTO {
    fn from(split: Split) -> Self {
        SplitDTO {
            id: split.id,
            description: split.description,
            participants: split.participants.join(","),
            default_currency: split.default_currency,
        }
    }
}

impl Into<Split> for SplitDTO {
    fn into(self) -> Split {
        Split {
            id: self.id,
            description: self.description,
            participants: self.participants.split(",").map(|s| s.to_string()).collect(),
            default_currency: self.default_currency,
        }
    }
}

pub struct CreateSplit {
    pub description: String,
    pub participants: Vec<String>,
    pub default_currency: String,
}

pub struct UpdateSplit {
    pub id: String,
    pub description: Option<String>,
    pub participants: Option<Vec<String>>,
    pub default_currency: Option<String>,
}

pub struct SplitRepo {}

impl SplitRepo {
    pub async fn get_all(db: &DB) -> Result<Vec<Split>, Error> {
        let split_dtos: Vec<SplitDTO> = sqlx::query_as("SELECT * FROM splits")
            .fetch_all(db.get_pool())
            .await?;

        Ok(split_dtos
            .into_iter()
            .map(|dto: SplitDTO| dto.into())
            .collect())
    }

    pub async fn get_by_id(db: &DB, id: String) -> Result<Split, Error> {
        let dto: SplitDTO = sqlx::query_as("SELECT * FROM splits WHERE id = $1")
            .bind(id)
            .fetch_one(db.get_pool())
            .await?;

        Ok(dto.into())
    }

    pub async fn create(db: &DB, split: CreateSplit) -> Result<String, Error> {
        let id = Self::generate_id();

        sqlx::query(
            "INSERT INTO splits (id, description, participants, default_currency) VALUES ($1, $2, $3, $4)",
        )
        .bind(id.clone())
        .bind(split.description)
        .bind(split.participants.join(","))
        .bind(split.default_currency)
        .execute(db.get_pool())
        .await?;

        Ok(id)
    }

    pub async fn update(db: &DB, split: UpdateSplit) -> Result<(), Error> {
        sqlx::query(r#"
        UPDATE splits 
        SET description = COALESCE($1, description),
            participants = COALESCE($2, participants), 
            default_currency = COALESCE($3, default_currency)
        WHERE id = $4
        "#)
            .bind(split.description)
            .bind(split.participants.map(|p| p.join(",")))
            .bind(split.default_currency)
            .bind(split.id)
            .execute(db.get_pool())
            .await?;

        Ok(())
    }

    pub async fn delete(db: &DB, id: String) -> Result<(), Error> {
        sqlx::query("DELETE FROM splits WHERE id = $1")
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
