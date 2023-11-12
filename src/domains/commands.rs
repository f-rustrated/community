use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccount {
    pub accountname: String,
}
