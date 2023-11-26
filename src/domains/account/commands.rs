use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccount {
    pub account_name: String,
    pub password: String,
}

pub struct SignInAccount {
    pub id: i64,
    pub password: String,
}
