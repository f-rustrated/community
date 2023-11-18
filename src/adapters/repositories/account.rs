use async_trait::async_trait;

use crate::domains::models::{repositories::AccountRepository, Account};

use super::SqlRepository;

#[async_trait]
impl AccountRepository for SqlRepository {
    async fn get(&self, id: i64) -> Account {
        todo!();
    }

    async fn add(&self, account: Account) {
        todo!();
    }

    async fn update(&self, account: Account) {
        todo!();
    }
}
