#[cfg(test)]
pub mod account_handler {
    use crate::{
        adapters::repositories::SqlRepository,
        domains::account::{Account, CreateAccount, SignInAccount},
        services::{
            account::{handlers::AccountHandler, repository::AccountRepository},
            responses::ApplicationResponse,
        },
    };

    #[tokio::test]
    async fn test_sign_up_account() {
        // given
        dotenv::dotenv().ok();
        let plain_password = "testPassword123!";
        let name = "test@community.com";
        let cmd = CreateAccount {
            email: name.to_string(),
            password: plain_password.to_string(),
        };
        let repo = SqlRepository::new().await;

        // when
        let mut handler = AccountHandler::new(SqlRepository::new().await);
        let Ok(ApplicationResponse::I64(account_id)) = handler.sign_up_account(cmd).await else {
            panic!("sign up failed");
        };
        let account = repo
            .get(account_id)
            .await
            .expect("No account has been inserted!");

        // then
        assert_eq!(account.name, name.to_string());
        assert_ne!(
            account.hashed_password,
            Account::create_password(plain_password).unwrap()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn test_sign_in_account_happy_case() {
        // given
        dotenv::dotenv().ok();
        let plain_password = "testPassword123!";
        let name = "test_sign_in@community.com";
        let create_cmd = CreateAccount {
            email: name.to_string(),
            password: plain_password.to_string(),
        };
        let mut handler = AccountHandler::new(SqlRepository::new().await);
        handler
            .sign_up_account(create_cmd)
            .await
            .expect("sign up failed");

        let sign_in_cmd = SignInAccount {
            email: name.to_string(),
            password: plain_password.to_string(),
        };

        // when
        let Ok(ApplicationResponse::String(token)) = handler.sign_in_account(sign_in_cmd).await
        else {
            panic!("failed");
        };

        // then
        assert!(!token.is_empty());
    }
}
