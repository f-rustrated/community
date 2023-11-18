use crate::adapters::repositories::SqlRepository;
use crate::domains::commands::CreateAccount;
use crate::domains::models::Account;

pub async fn create_account(command: CreateAccount) -> Account {
    let account = Account::new(command);
    SqlRepository::new().add(account.clone()).await
}