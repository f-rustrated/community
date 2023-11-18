use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::commands::CreateAccount;

#[derive(Debug, FromRow, Deserialize, Serialize)]
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
        Self {
            id: 0,
            uuid: Uuid::new_v4(),
            name: cmd.accountname,
            status: AccountStatus::Active,
            hashed_password: "".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Serialize)]
pub enum AccountStatus {
    Active,
    Deleted,
    Abnormal,
}

#[derive(Serialize)]
pub struct CommunityPost {
    id: i64,
    account_id: i64,
    title: String,
    thumbnail: Option<String>,
    category: PostCategory,
    body: String,
    status: PostStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub enum PostCategory {
    Default,
    Knowledge,
}

#[derive(Serialize)]
pub enum PostStatus {
    Created,
    Deleted,
    Edited,
}

pub mod repositories {
    use super::Account;
    use async_trait::async_trait;

    #[async_trait]
    pub trait AccountRepository {
        async fn get(&self, id: i64) -> Account;

        async fn add(&self, account: Account);

        async fn update(&self, account: Account);
    }
}
