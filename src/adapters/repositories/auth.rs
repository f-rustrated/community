use async_trait::async_trait;

use crate::{
    domains::account::Account,
    services::{
        account::repository::AccountRepository, cross_cutting_traits::TransactionUnitOfWork,
        responses::BaseError,
    },
};

use super::SqlRepository;

#[async_trait]
impl AccountRepository for SqlRepository {
    async fn get(&self, id: i64) -> Result<Account, BaseError> {
        todo!()
    }

    async fn add(&self, account: &Account) -> Result<(), BaseError> {
        todo!()
    }

    async fn update(&self, account: &Account) -> Result<(), BaseError> {
        todo!()
    }
}
