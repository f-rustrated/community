use async_trait::async_trait;

use crate::{domains::account::Account, services::responses::BaseError};

#[async_trait]
pub trait AccountRepository {
    async fn get(&self, id: i64) -> Result<Account, BaseError>;

    async fn add(&self, account: &Account) -> Result<(), BaseError>;

    async fn update(&self, account: &Account) -> Result<(), BaseError>;
}
