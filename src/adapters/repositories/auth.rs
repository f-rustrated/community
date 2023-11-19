use async_trait::async_trait;

use crate::{
    domains::auth::Account,
    services::{auth::repository::AccountRepository, cross_cutting_traits::TransactionUnitOfWork},
};

use super::SqlRepository;

#[async_trait]
impl AccountRepository for SqlRepository {
    async fn get(&self, id: i64) -> Account {
        todo!();
    }

    async fn add(&self, account: &Account) {
        todo!();
    }

    async fn update(&self, account: &Account) {
        todo!();
    }
}

#[async_trait]
impl TransactionUnitOfWork for SqlRepository {
    async fn begin(&self) {
        todo!()
    }
    async fn commit(&self) {
        todo!()
    }
    async fn rollback(&self) {
        todo!()
    }
}
