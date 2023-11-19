use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use self::commands::CreateAccount;
pub mod commands;

#[derive(Serialize)]
pub enum AccountStatus {
    Active,
    Deleted,
    Abnormal,
}

#[derive(Serialize)]
pub struct Account {
    id: i64,
    uuid: Uuid,
    name: String,
    status: AccountStatus,
    hashed_password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
