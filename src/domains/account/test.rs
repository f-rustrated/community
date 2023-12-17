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
    use crate::{
        config::config,
        domains::account::{Account, CreateAccount, JWTClaim},
    };
    use bcrypt;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

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

    #[tokio::test]
    async fn test_create_access_token() {
        // given
        let cmd = CreateAccount {
            email: "test_create_access_token@community.com".to_string(),
            password: "testPassword123!".to_string(),
        };
        let account = Account::new(&cmd).unwrap();

        // when
        let token = account.create_access_token().unwrap();

        // then
        let decoding_key = DecodingKey::from_secret(config().jwt_secret.as_bytes());
        let decoded =
            decode::<JWTClaim>(&token, &decoding_key, &Validation::new(Algorithm::HS256)).unwrap();
        assert_eq!(decoded.claims.sub, account.uuid.to_string());
    }
}
