#[cfg(test)]
pub mod account_handler {
    use crate::{
        adapters::repositories::SqlRepository,
        domains::account::commands::CreateAccount,
        services::{
            account::{handlers::AccountHandler, repository::AccountRepository},
            responses::ApplicationResponse,
        },
    };

    #[tokio::test]
    async fn test_sign_up_account() {
        // given
        dotenv::dotenv().ok();
        let plain_password = "test_password";
        let name = "test@community.com";
        let cmd = CreateAccount {
            email: name.to_string(),
            password: plain_password.to_string(),
        };
        let repo = SqlRepository::new().await;

        // when
        let mut handler = AccountHandler::new(SqlRepository::new().await);
        let Ok(ApplicationResponse::I64(account_id)) = handler.sign_up_account(cmd).await else {
            panic!("Fuck up!");
        };
        let account = repo
            .get(account_id)
            .await
            .expect("No account has been inserted!");

        // then
        assert_eq!(account.name, name.to_string());

        //TODO make it compare hash_password with hashed_plain_password
        assert_ne!(account.hashed_password, plain_password.to_string());
    }
}
