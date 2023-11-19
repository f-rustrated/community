use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use self::commands::CreateAccount;
pub mod commands;
pub(crate) mod repository;

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
}
