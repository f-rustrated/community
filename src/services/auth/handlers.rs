use super::repository::AccountRepository;
use crate::{
    domains::auth::{
        commands::{CreateAccount, SignInAccount},
        Account,
    },
    services::cross_cutting_traits::TransactionUnitOfWork,
};
use serde_json::Value;

struct AccountHandler<R> {
    repo: R,
}

// Transactional Handler
impl<R: AccountRepository + TransactionUnitOfWork> AccountHandler<R> {
    async fn create_account(&self, cmd: CreateAccount) {
        self.repo.begin().await;

        let account = Account::new(cmd);
        self.repo.add(&account).await;

        self.repo.commit().await;
    }
}

// Non-Transactional Handler
impl<R: AccountRepository> AccountHandler<R> {
    async fn sign_in_account(&self, cmd: SignInAccount) -> Value {
        let aggregate = self.repo.get(cmd.id).await;

        if !aggregate.verify_password(&cmd.password) {
            todo!()
        }
        aggregate.create_access_token()
    }
}
