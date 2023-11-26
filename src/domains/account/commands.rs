use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateAccount {
    pub name: String,
    pub password: String,
}

pub struct SignInAccount {
    pub id: i64,
    pub password: String,
}
