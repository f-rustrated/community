use serde::{de::DeserializeOwned, Serialize};

pub mod account;

pub mod post;
#[cfg(test)]
pub mod test;

pub trait TAggregate: Default + Serialize + DeserializeOwned + Sync + Send {
    type Command;
    type Event: TEvent;
    type Error: std::error::Error;

    fn handle(&mut self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error>;
    fn apply(&mut self, event: Self::Event);
}

pub trait TEvent:
    Serialize + DeserializeOwned + Clone + PartialEq + std::fmt::Debug + Sync + Send
{
    /// for event upcasting.
    fn event_type(&self) -> String;
    /// used for event upcasting.
    fn event_version(&self) -> String;
    fn aggregate_type(&self) -> String;
}

pub trait TEventStore<A: TAggregate>: Sync + Send {
    type AC: TAggregateContext<A>;
    fn load_events(
        &self,
        aggregate_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<A::Event>, A::Error>> + Send;
    fn load_aggregate(
        &self,
        aggregate_id: &str,
    ) -> impl std::future::Future<Output = Result<Self::AC, A::Error>> + Send {
        async {
            let events = self.load_events(aggregate_id).await?;
            let mut aggregate = A::default();
            let mut current_sequence = 0;
            events.into_iter().for_each(|event| {
                current_sequence += 1;
                aggregate.apply(event)
            });
            Ok(Self::AC::new(aggregate_id, aggregate, current_sequence))
        }
    }
    fn commit(
        &self,
        events: Vec<A::Event>,
        context: Self::AC,
    ) -> impl std::future::Future<Output = Result<(), A::Error>> + Send;
}

pub trait TAggregateContext<A>
where
    A: TAggregate,
{
    /// The aggregate instance with all state loaded.
    fn aggregate(&self) -> &A;
    fn new(aggregate_id: &str, aggregate: A, current_sequence: usize) -> Self;
}

pub struct AggregateContext<A>
where
    A: TAggregate,
{
    /// The aggregate ID of the aggregate instance that has been loaded.
    pub aggregate_id: String,
    /// The current state of the aggregate instance.
    pub aggregate: A,
    /// The last committed event sequence number for this aggregate instance.
    pub current_sequence: usize,
}

impl<A: TAggregate> TAggregateContext<A> for AggregateContext<A> {
    fn aggregate(&self) -> &A {
        &self.aggregate
    }

    fn new(aggregate_id: &str, aggregate: A, current_sequence: usize) -> Self {
        Self {
            aggregate_id: aggregate_id.to_string(),
            aggregate,
            current_sequence,
        }
    }
}
