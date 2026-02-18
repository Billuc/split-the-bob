use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    MigrateError(sqlx::migrate::MigrateError),
    TemplateError(askama::Error),
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

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        Error::TemplateError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SqlxError(err) => write!(f, "Database error: {}", err),
            Error::MigrateError(err) => write!(f, "Migration error: {}", err),
            Error::TemplateError(err) => write!(f, "Template rendering error: {}", err),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::SqlxError(err) => {
                eprintln!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::MigrateError(err) => {
                eprintln!("Migration error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::TemplateError(err) => {
                eprintln!("Template rendering error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}
