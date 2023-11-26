use super::repository::AccountRepository;
use crate::{
    domains::account::{
        commands::{CreateAccount, SignInAccount},
        Account,
    },
    services::{cross_cutting_traits::TransactionUnitOfWork, responses::ServiceError},
};
use serde_json::Value;
use crate::services::responses::BaseError;

#[derive(Clone)]
pub(crate) struct AccountHandler<R> {
    pub(crate) repo: R,
}

// Transactional Handler
impl<R: AccountRepository + TransactionUnitOfWork> AccountHandler<R> {
    pub(crate) async fn sign_up_account(
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

    pub(crate) async fn sign_in_account(&self, cmd: SignInAccount) -> Result<String, ServiceError> {
        match self.repo.get_by_email(cmd.email).await {
            Ok(account) =>  {
              if account.verify_password(&cmd.password) {
                  return Ok("token".to_owned())
              }
            }
            _ => {}
        }

        Err(ServiceError::AuthenticationError("Invalid email or password".to_owned()))
    }
}
