use std::marker::PhantomData;

use crate::domains::TAggregate;

#[derive(Default)]
pub struct TestFrameWork<A: TAggregate>(PhantomData<A>);

impl<A> TestFrameWork<A>
where
    A: TAggregate,
{
    #[must_use]
    pub fn given_no_previous_events(self) -> AggregateTestExecutor<A> {
        AggregateTestExecutor::new(Vec::new())
    }
    #[must_use]
    pub fn given(self, events: Vec<A::Event>) -> AggregateTestExecutor<A> {
        AggregateTestExecutor::new(events)
    }
}

pub struct AggregateTestExecutor<A>
where
    A: TAggregate,
{
    events: Vec<A::Event>,
}
impl<A> AggregateTestExecutor<A>
where
    A: TAggregate,
{
    pub fn new(events: Vec<A::Event>) -> Self {
        Self { events }
    }

    pub fn when(self, command: A::Command) -> AggregateResultValidator<A> {
        let mut aggregate = A::default();
        self.events
            .into_iter()
            .for_each(|event| aggregate.apply(event));
        let result = aggregate.handle(command);
        AggregateResultValidator::new(result)
    }
}

pub struct AggregateResultValidator<A>
where
    A: TAggregate,
{
    result: Result<Vec<A::Event>, A::Error>,
}

impl<A: TAggregate> AggregateResultValidator<A> {
    pub fn then_expect_events(self, expected_events: Vec<A::Event>) {
        let events = match self.result {
            Ok(expected_events) => expected_events,
            Err(err) => {
                panic!("expected success, received aggregate error: '{:?}'", err);
            }
        };
        assert_eq!(events, expected_events);
    }

    pub fn then_expect_error_message(self, error_message: &str) {
        match self.result {
            Ok(events) => {
                panic!("expected error, received events: '{:?}'", events);
            }
            Err(err) => assert_eq!(err.to_string(), error_message.to_string()),
        };
    }

    pub(crate) fn new(result: Result<Vec<A::Event>, A::Error>) -> Self {
        Self { result }
    }
}

#[cfg(test)]
pub mod account {
    use crate::domains::account::{
        test::TestFrameWork, Account, AccountError, AccountEvent, CreateAccount, PasswordPolicy,
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
