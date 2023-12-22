use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum AccountError {
    HashLibError(String),
    InvalidPassword(PasswordPolicy),
    AuthenticationError(String),
}

impl std::error::Error for AccountError {}

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashLibError(_) => write!(f, "hash library error!"),
            Self::InvalidPassword(_) => write!(f, "invalid password given!"),
            Self::AuthenticationError(_) => write!(f, "failed to authenticate"),
        }
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

impl From<PasswordPolicy> for AccountError {
    fn from(value: PasswordPolicy) -> Self {
        Self::InvalidPassword(value)
    }
}
