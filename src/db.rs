use sqlx::{FromRow, sqlite::SqlitePool, sqlite::SqliteRow};

#[derive(Clone)]
pub struct DB {
    pool: SqlitePool,
}

#[derive(Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    MigrateError(sqlx::migrate::MigrateError),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SqlxError(err)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Error::MigrateError(err)
    }
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
