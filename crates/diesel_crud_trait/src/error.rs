use diesel::result::Error;
use thiserror::Error;

/// Error type for CRUD operations
#[derive(Debug, PartialEq, Error)]
pub enum CrudError {
    #[error("Resource not found")]
    NotFound,
    #[error("Database pool error: {0}")]
    PoolError(String),
    #[error(transparent)]
    Other(Error),
}

impl From<Error> for CrudError {
    fn from(error: Error) -> Self {
        match error {
            Error::NotFound => CrudError::NotFound,
            _ => CrudError::Other(error),
        }
    }
}
