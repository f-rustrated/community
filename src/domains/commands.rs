use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccount {
    pub accountname: String,
}

#[derive(Deserialize)]
pub struct CreatePost {
    pub postname: String,
}
