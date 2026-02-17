use crate::error::Error;
use sqlx::{FromRow, sqlite::SqlitePool, sqlite::SqliteRow};

#[derive(Clone)]
pub struct DB {
    pool: SqlitePool,
}

impl DB {
    pub async fn new(db_url: &str) -> Result<DB, Error> {
        let pool = SqlitePool::connect(db_url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(DB { pool: pool })
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
