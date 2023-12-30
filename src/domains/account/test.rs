#[cfg(test)]
pub mod account {
    use crate::domains::{
        account::{Account, AccountError, AccountEvent, CreateAccount, PasswordPolicy},
        test::TestFrameWork,
    };
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

    #[test]
    fn test_create_account() {
        let cmd = CreateAccount {
            email: "test_create_access_token@community.com".to_string(),
            password: "testPassword123!".to_string(),
        }
        .into();

        let Ok(res) = TestFrameWork::<Account>::default()
            .given_no_previous_events()
            .when(cmd)
            .result
        else {
            panic!("failed!")
        };

        assert!(!res.is_empty());
        assert_eq!(res.len(), 1);
        let AccountEvent::AccountCreated {
            hashed_password, ..
        } = res.first().unwrap().clone()
        else {
            panic!("Wrong event!")
        };

        assert_ne!(hashed_password, "testPassword123!".to_string());
    }

    #[tokio::test]
    async fn test_validate_password() {
        // given
        let fail_cases: [(&str, AccountError); 5] = [
            ("!1Short", PasswordPolicy::NotEnoughChars.into()),
            ("UPPERCASEONLY", PasswordPolicy::AtLeastOneLower.into()),
            ("lowercaseonly", PasswordPolicy::AtLeastOneUpper.into()),
            ("lowerUPPER", PasswordPolicy::AtLeastOneNumber.into()),
            (
                "noSpecialChar123",
                PasswordPolicy::AtLeastOneSpecialChar.into(),
            ),
        ];
        let pass_case = "MyPassword1@";

        for case in &fail_cases {
            let result = Account::validate_password(case.0);
            assert_eq!(result.unwrap_err(), case.1);
        }
        Account::validate_password(pass_case).expect("validation failed");
    }
}
