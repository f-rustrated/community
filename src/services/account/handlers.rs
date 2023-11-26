use super::repository::AccountRepository;
use crate::{
    domains::account::{
        commands::{CreateAccount, SignInAccount},
        Account,
    },
    services::{cross_cutting_traits::TransactionUnitOfWork, responses::ServiceError},
};
use serde_json::Value;

#[derive(Clone)]
pub(crate) struct AccountHandler<R> {
    pub(crate) repo: R,
}

// Transactional Handler
impl<R: AccountRepository + TransactionUnitOfWork> AccountHandler<R> {
    pub(crate) async fn create_account(
        &mut self,
        cmd: CreateAccount,
    ) -> Result<Account, ServiceError> {
        self.repo.begin().await?;

        let account = self.repo.add(&Account::new(&cmd)).await?;

        self.repo.commit().await?;

        Ok(account)
    }
}

// Non-Transactional Handler
impl<R: AccountRepository> AccountHandler<R> {
    pub(crate) async fn get_account(&self, id: i64) -> Result<Account, ServiceError> {
        Ok(self.repo.get(id).await?)
    }

    async fn sign_in_account(&self, cmd: SignInAccount) -> Result<Value, ServiceError> {
        todo!()
    }
}
