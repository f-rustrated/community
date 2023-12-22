use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub enum AccountCommand {
    CreateAccount(CreateAccount),
    SignInAccount(SignInAccount),
}

#[derive(Deserialize, Clone)]
pub struct CreateAccount {
    pub email: String,
    pub password: String,
}
impl From<CreateAccount> for AccountCommand {
    fn from(value: CreateAccount) -> Self {
        Self::CreateAccount(value)
    }
}

#[derive(Deserialize, Clone)]
pub struct SignInAccount {
    pub email: String,
    pub password: String,
}

impl From<SignInAccount> for AccountCommand {
    fn from(value: SignInAccount) -> Self {
        Self::SignInAccount(value)
    }
}
