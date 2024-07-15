#[derive(Debug)]
pub(crate) enum ProcessingError {
    SerdeError(serde_json::Error),
    PostgresError(tokio_postgres::Error),
    NotImplemented,
}

impl From<serde_json::Error> for ProcessingError {
    fn from(value: serde_json::Error) -> Self {
        ProcessingError::SerdeError(value)
    }
}

impl From<tokio_postgres::Error> for ProcessingError {
    fn from(value: tokio_postgres::Error) -> Self {
        ProcessingError::PostgresError(value)
    }
}