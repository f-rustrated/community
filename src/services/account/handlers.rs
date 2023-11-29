use super::repository::AccountRepository;
use crate::{
    domains::account::{
        commands::{SignUpAccount, SignInAccount},
        Account,
    },
    services::{cross_cutting_traits::TransactionUnitOfWork, responses::ServiceError},
};
use crate::domains::account::commands::UpdateAccount;

#[derive(Clone)]
pub(crate) struct AccountHandler<R> {
    pub(crate) repo: R,
}

// Transactional Handler
impl<R: AccountRepository + TransactionUnitOfWork> AccountHandler<R> {
    pub(crate) async fn sign_up_account(
        &mut self,
        cmd: SignUpAccount,
    ) -> Result<Account, ServiceError> {
        self.repo.begin().await?;

        let account = self.repo.add(&Account::new(&cmd)).await?;

        self.repo.commit().await?;

        Ok(account)
    }

    pub(crate) async fn update_account(
        &mut self,
        cmd: UpdateAccount,
    ) -> Result<(), ServiceError> {
        self.repo.begin().await?;

        let account = self.repo.get_by_uuid(cmd.uuid).await?;
        let updated_account = Account::merge(&account, &cmd);
        return match self.repo.update(&updated_account).await {
            Ok(_) => {
                self.repo.commit().await?;
                Ok(())
            }
            Err(err) => {
                self.repo.rollback().await?;
                Err(ServiceError::from(err))
            }
        }
    }
}

// Non-Transactional Handler
impl<R: AccountRepository> AccountHandler<R> {
    pub(crate) async fn get_account(&self, id: i64) -> Result<Account, ServiceError> {
        Ok(self.repo.get(id).await?)
    }

    pub(crate) async fn sign_in_account(&self, cmd: SignInAccount) -> Result<String, ServiceError> {
        match self.repo.get_by_email(cmd.email).await {
            Ok(account) => {
                if account.verify_password(&cmd.password) {
                    return Ok("token".to_owned());
                }
            }
            _ => {}
        }

        Err(ServiceError::AuthenticationError("Invalid email or password".to_owned()))
    }
}
