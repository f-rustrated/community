#[cfg(test)]
pub mod account_handler {
    use crate::{
        adapters::repositories::SqlRepository,
        domains::account::{
            commands::{CreateAccount, SignInAccount},
            Account,
        },
        services::{
            account::{
                handlers::validate_password, handlers::AccountHandler,
                repository::AccountRepository,
            },
            responses::{ApplicationResponse, PasswordPolicy},
        },
    };

    #[tokio::test]
    async fn test_validate_password() {
        // given
        let fail_cases: [(&str, PasswordPolicy); 5] = [
            ("!1Short", PasswordPolicy::NotEnoughChars),
            ("UPPERCASEONLY", PasswordPolicy::AtLeastOneLower),
            ("lowercaseonly", PasswordPolicy::AtLeastOneUpper),
            ("lowerUPPER", PasswordPolicy::AtLeastOneNumber),
            ("noSpecialChar123", PasswordPolicy::AtLeastOneSpecialChar),
        ];
        let pass_case = "MyPassword1@";

        for case in &fail_cases {
            let result = validate_password(case.0);
            assert_eq!(result.unwrap_err(), case.1);
        }
        validate_password(pass_case).expect("validation failed");
    }

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
        assert_eq!(
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
        let repo = SqlRepository::new().await;
        let mut handler = AccountHandler::new(SqlRepository::new().await);
        let Ok(ApplicationResponse::I64(account_id)) = handler.sign_up_account(create_cmd).await
        else {
            panic!("sign up failed");
        };

        let sign_in_cmd = SignInAccount {
            email: name.to_string(),
            password: plain_password.to_string(),
        };

        // when
        let Ok(ApplicationResponse::Json(token)) = handler.sign_in_account(sign_in_cmd).await
        else {
            panic!("failed");
        };

        // then
        assert!(token.is_object());
    }
}
