use diesel::result::{DatabaseErrorKind, Error as DieselError};
use std::fmt;

// parent error type
#[derive(Debug)]
pub enum ServiceError {
    StorageError(StorageError),
}

impl From<StorageError> for ServiceError {
    fn from(error: StorageError) -> Self {
        ServiceError::StorageError(error)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::StorageError(error) => error.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum StorageError {
    ConnectionFailed(String),
    DuplicateEntry(String),
    OtherDatabaseError(String),
    SelectionFailed(String),
    TaskJoinError(String),
    // Add other error variants as needed
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            StorageError::DuplicateEntry(msg) => write!(f, "Duplicate entry: {}", msg),
            StorageError::OtherDatabaseError(msg) => write!(f, "Other database error: {}", msg),
            StorageError::SelectionFailed(msg) => write!(f, "Selection failed: {}", msg),
            StorageError::TaskJoinError(msg) => write!(f, "Task join error: {}", msg),
        }
    }
}

impl From<DieselError> for StorageError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                let details = info.message().to_string();
                StorageError::DuplicateEntry(details)
            },
            other => StorageError::OtherDatabaseError(other.to_string()),
        }
    }
}