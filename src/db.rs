use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct DB {
    pool: SqlitePool,
}

#[derive(Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    MigrateError(sqlx::migrate::MigrateError),
}

impl DB {
    pub async fn new(db_url: &str) -> Result<DB, Error> {
        let pool = SqlitePool::connect(db_url)
            .await
            .map_err(Error::SqlxError)?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(Error::MigrateError)?;

        Ok(DB { pool: pool })
    }
}
