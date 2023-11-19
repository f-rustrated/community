use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccount {
    pub accountname: String,
}

pub struct SignInAccount {
    pub id: i64,
    pub password: String,
}
