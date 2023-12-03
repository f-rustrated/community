// TODO define service response for service objects

use serde::Serialize;

use crate::domains::account::response::AccountResponse;

// TODO define service error for fallible service operation
#[derive(Debug, Serialize)]
pub enum BaseError {
    DatabaseError,
    NotFound,
    ConstraintError,
    TransactionError,
    InternalError,
}

#[derive(Debug, Serialize)]
pub enum ServiceError {
    BaseError(BaseError),
    AuthenticationError(String),
}

impl From<BaseError> for ServiceError {
    fn from(value: BaseError) -> Self {
        ServiceError::BaseError(value)
    }
}

#[derive(Serialize)]
pub enum ApplicationResponse {
    Account(AccountResponse),
    String(String),
}

impl From<String> for ApplicationResponse {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<AccountResponse> for ApplicationResponse {
    fn from(value: AccountResponse) -> Self {
        Self::Account(value)
    }
}
