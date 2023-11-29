use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct SignUpAccount {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateAccount {
    pub uuid: Uuid,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct SignInAccount {
    pub email: String,
    pub password: String,
}
