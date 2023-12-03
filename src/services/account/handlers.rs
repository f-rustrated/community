use super::repository::AccountRepository;
use crate::{
    domains::account::{
        commands::{CreateAccount, SignInAccount},
        response::AccountResponse,
        Account,
    },
    services::{
        cross_cutting_traits::TransactionUnitOfWork,
        responses::{ApplicationResponse, ServiceError},
    },
};

#[derive(Clone)]
pub struct AccountHandler<R> {
    repo: R,
}

impl<R> AccountHandler<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

// Transactional Handler
impl<R: AccountRepository + TransactionUnitOfWork> AccountHandler<R> {
    pub async fn sign_up_account(
        &mut self,
        cmd: CreateAccount,
    ) -> Result<ApplicationResponse, ServiceError> {
        self.repo.begin().await?;

        let account = self.repo.add(&Account::new(&cmd)).await?;

        self.repo.commit().await?;

        Ok(Into::<AccountResponse>::into(account).into())
    }
}

// Non-Transactional Handler
impl<R: AccountRepository> AccountHandler<R> {
    pub async fn get_account(&self, id: i64) -> Result<Account, ServiceError> {
        Ok(self.repo.get(id).await?)
    }

    pub async fn sign_in_account(
        &self,
        cmd: SignInAccount,
    ) -> Result<ApplicationResponse, ServiceError> {
        self.repo.get_by_email(cmd.email).await.map_err(|err| {
            tracing::error!("{:?}", err);
            ServiceError::AuthenticationError("Invalid email or password".to_owned())
        })?;
        Ok("token".to_owned().into())
    }
}
