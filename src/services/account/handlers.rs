use super::repository::AccountRepository;
use crate::{
    domains::account::{
        commands::{CreateAccount, SignInAccount},
        Account,
    },
    services::{
        cross_cutting_traits::TransactionUnitOfWork,
        responses::{ApplicationResponse, PasswordPolicy, ServiceError},
    },
};
use regex::Regex;

#[derive(Clone)]
pub struct AccountHandler<R> {
    repo: R,
}

const MIN_PASSWORD_LEN: usize = 8;

pub(crate) fn validate_password(password: &str) -> Result<(), PasswordPolicy> {
    if password.len() < MIN_PASSWORD_LEN {
        return Err(PasswordPolicy::NotEnoughChars);
    }

    // TODO: compile regex once and reuse
    let pattern = Regex::new(r#"[a-z]"#).unwrap();
    if !pattern.is_match(password) {
        return Err(PasswordPolicy::AtLeastOneLower);
    }

    let pattern = Regex::new(r#"[A-Z]"#).unwrap();
    if !pattern.is_match(password) {
        return Err(PasswordPolicy::AtLeastOneUpper);
    }

    let pattern = Regex::new(r#"\d"#).unwrap();
    if !pattern.is_match(password) {
        return Err(PasswordPolicy::AtLeastOneNumber);
    }

    let pattern = Regex::new(r#"[!@%^&*?_-]"#).unwrap();
    if !pattern.is_match(password) {
        return Err(PasswordPolicy::AtLeastOneSpecialChar);
    }

    Ok(())
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

        match validate_password(&cmd.password) {
            Ok(_) => {}
            Err(password_policy) => {
                return Err(ServiceError::InvalidPassword(password_policy));
            }
        }

        let account = &Account::new(&cmd)?;
        let account_id = self.repo.add(account).await?;

        self.repo.commit().await?;

        Ok(ApplicationResponse::I64(account_id))
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
        let account = self.repo.get_by_email(cmd.email).await?;

        account.verify_password(&cmd.password)?;

        let token = account.create_access_token();
        Ok(token.into())
    }
}
