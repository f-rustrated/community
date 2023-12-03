use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

use self::commands::CreateAccount;

pub mod commands;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Comment {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) target_id: String,
    pub(crate) target_type: char,
    pub(crate) message: String,
    pub(crate) edited_yn: char,
    pub(crate) deleted_yn: char,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

impl Comment {
    pub(crate) fn save(cmd: &saveComment) {
        Self {
            id: 0,
            account_id: &saveComment.account_id,
            target_id: &saveComment.target_id,
            target_type: &saveComment.target_type,
            message: &saveComment.message,
            edited_yn: 'N',
            deleted_yn: 'N',
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc()
        }
    }

    pub(crate) fn delete(cmd: &deleteComment) {
        todo!()
    }
}
