use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub mod response;
use uuid::Uuid;

use self::commands::CreateAccount;

pub mod commands;
#[cfg(test)]
pub mod test;

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
    pub(crate) fn new(cmd: &CreateAccount) -> Self {
        Self {
            id: 0,
            uuid: Uuid::new_v4(),
            name: cmd.email.to_string(),
            status: AccountStatus::Active,
            hashed_password: Self::create_password(&cmd.password),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // * For the given aggregate, verify passed password
    pub(crate) fn verify_password(&self, plaintext_password: &str) -> bool {
        plaintext_password == "hashed_password"
    }

    // * static method to create hashed_password
    pub(crate) fn create_password(plaintext_password: &str) -> String {
        String::from("hashed_password")
    }
    pub(crate) fn create_access_token(&self) -> Value {
        todo!()
    }
}
