use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use sqlx;

use self::commands::CreateAccount;
pub mod commands;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_status")]
pub enum AccountStatus {
    Active,
    Deleted,
    Abnormal,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: i64,
    pub uuid: Uuid,
    pub name: String,
    pub status: AccountStatus,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Account {
    pub(crate) fn new(cmd: CreateAccount) -> Self {
        todo!()
    }

    // * For the given aggregate, verify passed password
    pub(crate) fn verify_password(&self, plaintext_password: &str) -> bool {
        todo!()
    }

    // * static method to create hashed_password
    pub(crate) fn create_password(plaintext_password: &str) -> String {
        todo!()
    }
    pub(crate) fn create_access_token(&self) -> Value {
        todo!()
    }
}
