use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateAccount {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct SignInAccount {
    pub email: String,
    pub password: String,
}
