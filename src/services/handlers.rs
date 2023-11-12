use crate::domains::{
    commands::CreateAccount,
    models::{repositories::AccountRepository, Account},
};

struct AccountHandler {
    repo: Box<dyn AccountRepository>,
}

impl AccountHandler {
    async fn create_account(&self, cmd: CreateAccount) {
        let account = Account::new(cmd);
        self.repo.add(account).await;
    }
}
