use serde::Serialize;

use super::Account;

#[derive(Serialize)]
pub enum AccountResponse {
    Account(Account),
}

impl From<Account> for AccountResponse {
    fn from(value: Account) -> Self {
        Self::Account(value)
    }
}
