use crate::domains::auth::{commands::CreateAccount, repository::AccountRepository, Account};

struct AccountHandler {
    repo: Box<dyn AccountRepository>,
}

impl AccountHandler {
    async fn create_account(&self, cmd: CreateAccount) {
        let account = Account::new(cmd);
        self.repo.add(account).await;
    }
}
