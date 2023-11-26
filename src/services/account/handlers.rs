use super::repository::AccountRepository;
use crate::{
    domains::account::{
        commands::{CreateAccount, SignInAccount},
        Account,
    },
    services::{cross_cutting_traits::TransactionUnitOfWork, responses::ServiceError},
};
use serde_json::Value;
use uuid::Uuid;

pub struct AccountHandler<R> {
    repo: R,
}

impl<R: AccountRepository> AccountHandler<R> {
    pub fn new(repo: R) -> AccountHandler<R> {
        AccountHandler { repo: repo }
    }
}

// Transactional Handler
impl<R: AccountRepository + TransactionUnitOfWork> AccountHandler<R> {
    pub async fn create_account(&mut self, cmd: CreateAccount) -> Result<(), ServiceError> {
        self.repo.begin().await?;

        let account = Account::new(cmd);
        self.repo.add(&account).await?;

        self.repo.commit().await?;

        Ok(())
    }
}

// Non-Transactional Handler
impl<R: AccountRepository> AccountHandler<R> {
    pub async fn get_account(&mut self, uuid: Uuid) -> Result<Account, ServiceError> {
        // let aggregate = self.repo.get_by_uuid(uuid).await?;
        match self.repo.get_by_uuid(uuid).await {
            Ok(account) => Ok(account),
            Err(err) => Err(ServiceError::from(err)),
        }
    }

    async fn sign_in_account(&self, cmd: SignInAccount) -> Result<Value, ServiceError> {
        let aggregate = self.repo.get(cmd.id).await?;

        if !aggregate.verify_password(&cmd.password) {
            todo!()
        }
        Ok(aggregate.create_access_token())
    }
}
