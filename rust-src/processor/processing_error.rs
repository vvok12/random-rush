#[derive(Debug)]
pub(crate) enum ProcessingError {
    SerdeError(serde_json::Error),
    PostgresError(sqlx::Error),
    NotImplemented,
}

impl From<serde_json::Error> for ProcessingError {
    fn from(value: serde_json::Error) -> Self {
        ProcessingError::SerdeError(value)
    }
}

impl From<sqlx::Error> for ProcessingError {
    fn from(value: sqlx::Error) -> Self {
        ProcessingError::PostgresError(value)
    }
}