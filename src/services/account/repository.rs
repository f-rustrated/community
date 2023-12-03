use async_trait::async_trait;

use crate::{domains::account::Account, services::responses::BaseError};

#[async_trait]
pub trait AccountRepository {
    async fn get(&self, id: i64) -> Result<Account, BaseError>;

    async fn get_by_email(&self, email: String) -> Result<Account, BaseError>;

    async fn add(&mut self, account: &Account) -> Result<i64, BaseError>;

    async fn update(&mut self, account: &Account) -> Result<(), BaseError>;
}
