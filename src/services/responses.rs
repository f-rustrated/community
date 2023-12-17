// TODO define service response for service objects

use serde::Serialize;
use serde_json::Value;

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
    InvalidPassword(PasswordPolicy),
    HashLibError(String),
    JWTError(String),
}

impl From<BaseError> for ServiceError {
    fn from(value: BaseError) -> Self {
        ServiceError::BaseError(value)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum PasswordPolicy {
    NotEnoughChars = 1,
    AtLeastOneLower = 2,
    AtLeastOneUpper = 3,
    AtLeastOneNumber = 4,
    AtLeastOneSpecialChar = 5,
}

#[derive(Serialize)]
pub enum ApplicationResponse {
    Account(AccountResponse),
    String(String),
    I64(i64),
    Json(Value),
}

impl From<String> for ApplicationResponse {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Value> for ApplicationResponse {
    fn from(value: Value) -> Self {
        Self::Json(value)
    }
}

impl From<AccountResponse> for ApplicationResponse {
    fn from(value: AccountResponse) -> Self {
        Self::Account(value)
    }
}
