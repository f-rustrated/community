use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub mod response;
use bcrypt;
use uuid::Uuid;

use crate::services::responses::ServiceError;

use self::commands::CreateAccount;

pub mod commands;
#[cfg(test)]
pub mod test;

// TODO: decide salt mechanism later
const SALT: [u8; 16] = [
    49, 129, 3, 11, 159, 22, 1, 194, 94, 245, 142, 24, 21, 91, 99, 19,
];

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Deleted,
    Abnormal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub(crate) id: i64,
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) status: AccountStatus,
    pub(crate) hashed_password: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl Account {
    const ID_NOT_GENERATED: i64 = 0;

    pub(crate) fn new(cmd: &CreateAccount) -> Result<Self, ServiceError> {
        let hashed_password = Self::create_password(&cmd.password)?;

        Ok(Self {
            id: Account::ID_NOT_GENERATED,
            uuid: Uuid::new_v4(),
            name: cmd.email.to_string(),
            status: AccountStatus::Active,
            hashed_password: hashed_password,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // * For the given aggregate, verify passed password
    pub(crate) fn verify_password(&self, plain_password: &str) -> Result<(), ServiceError> {
        let is_verified = bcrypt::verify(plain_password, &self.hashed_password).map_err(|err| {
            eprintln!("bcrypt verify failed: {}", err);
            ServiceError::HashLibError(err.to_string())
        })?;
        if !is_verified {
            return Err(ServiceError::AuthenticationError(
                "password doesn't match".to_string(),
            ));
        }
        Ok(())
    }

    // * static method to create hashed_password
    pub(crate) fn create_password(plaintext_password: &str) -> Result<String, ServiceError> {
        match bcrypt::hash_with_salt(plaintext_password, bcrypt::DEFAULT_COST, SALT) {
            Ok(hash_parts) => Ok(hash_parts.to_string()),
            Err(err) => {
                eprintln!("bcrypt hash failed: {}", err);
                Err(ServiceError::HashLibError(err.to_string()))
            }
        }
    }

    pub(crate) fn create_access_token(&self) -> Value {
        todo!()
    }
}
