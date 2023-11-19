use async_trait::async_trait;

use super::Account;

#[async_trait]
pub trait AccountRepository {
    async fn get(&self, id: i64) -> Account;

    async fn add(&self, account: Account);

    async fn update(&self, account: Account);
}
