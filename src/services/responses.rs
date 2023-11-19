// TODO define service response for service objects

// TODO define service error for fallible service operation
#[derive(Debug)]
pub enum BaseError {
    DatabaseConnectionFailed,
    NotFound,
    ContraintError,
    TransactionError,
}

#[derive(Debug)]
pub enum ServiceError {
    BaseError(BaseError),
}

impl From<BaseError> for ServiceError {
    fn from(value: BaseError) -> Self {
        ServiceError::BaseError(value)
    }
}
