use super::Account;

pub fn account_create_helper(password: String, email: String) -> Account {
    crate::domains::account::Account::new(&crate::domains::account::commands::CreateAccount {
        password,
        email,
    })
}
