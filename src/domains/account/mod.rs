use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;
use crate::domains::account::commands::UpdateAccount;
use bcrypt::{DEFAULT_COST};

use self::commands::SignUpAccount;

pub mod commands;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
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
    pub(crate) fn new(cmd: &SignUpAccount) -> Self {
        Self {
            id: 0,
            uuid: Uuid::new_v4(),
            name: cmd.email.to_string(),
            status: AccountStatus::Active,
            hashed_password: Self::hash_password(&cmd.password),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub(crate) fn merge(account: &Account, cmd: &UpdateAccount) -> Account {
        Account {
            id: account.id,
            uuid: account.uuid,
            name: account.name.to_string(),
            status: account.status.clone(),
            hashed_password: Self::hash_password(&cmd.password),
            created_at: account.created_at,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub(crate) fn verify_password(&self, plaintext_password: &str) -> bool {
        match bcrypt::verify(plaintext_password, &self.hashed_password) {
            Ok(true) => true,
            _ => false
        }
    }

    pub(crate) fn hash_password(plaintext_password: &str) -> String {
        let hashed_password = bcrypt::hash_with_result(plaintext_password, DEFAULT_COST).unwrap();
        return hashed_password.to_string();
    }
    pub(crate) fn create_access_token(&self) -> Value {
        todo!()
    }
}
