#[derive(Debug)]
pub(crate) enum ProcessingError {
    SerdeError(serde_json::Error),
    NotImplemented,
}

impl From<serde_json::Error> for ProcessingError {
    fn from(value: serde_json::Error) -> Self {
        ProcessingError::SerdeError(value)
    }
}