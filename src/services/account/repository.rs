use crate::{
    domains::account::{Account, AccountEvent},
    services::responses::BaseError,
};
use std::future::Future;

pub trait AccountRepository {
    fn get(&self, id: i64) -> impl Future<Output = Result<Account, BaseError>> + Send;

    fn get_by_email(
        &self,
        email: String,
    ) -> impl Future<Output = Result<Account, BaseError>> + Send;

    fn add(
        &mut self,
        account: &[AccountEvent],
    ) -> impl Future<Output = Result<i64, BaseError>> + Send;

    fn update(&mut self, account: &Account) -> impl Future<Output = Result<(), BaseError>> + Send;
}
