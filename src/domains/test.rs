use std::marker::PhantomData;

use super::TAggregate;

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
    pub result: Result<Vec<A::Event>, A::Error>,
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
