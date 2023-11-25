use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, FromRow};
use uuid::Uuid;

use self::commands::CreateAccount;

pub mod commands;

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Deleted,
    Abnormal,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Account {
    pub(crate) id: i64,
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) status: AccountStatus,
    pub(crate) hashed_password: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

impl Account {
    pub(crate) fn new(cmd: &CreateAccount) -> Self {
        Self {
            id: 0,
            uuid: Uuid::new_v4(),
            name: cmd.account_name.to_string(),
            status: AccountStatus::Active,
            hashed_password: Self::create_password(&cmd.password),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    // * For the given aggregate, verify passed password
    pub(crate) fn verify_password(&self, plaintext_password: &str) -> bool {
        todo!()
    }

    // * static method to create hashed_password
    pub(crate) fn create_password(plaintext_password: &str) -> String {
        return String::from("hashed_password");
    }
    pub(crate) fn create_access_token(&self) -> Value {
        todo!()
    }
}
