use crate::services::responses::ServiceError;

use super::Account;

pub fn account_create_helper(password: String, email: String) -> Result<Account, ServiceError> {
    crate::domains::account::Account::new(&crate::domains::account::commands::CreateAccount {
        password,
        email,
    })
}

#[cfg(test)]
pub mod account {
    use crate::domains::account::Account;
    use bcrypt;

    #[tokio::test]
    async fn test_create_password() {
        // given
        let plain_password = "hello_world";

        // when
        let hashed = Account::create_password(plain_password).unwrap();
        let is_verified = bcrypt::verify(plain_password, &hashed).unwrap();

        // then
        assert!(is_verified);
    }
}
