use serde::{de::DeserializeOwned, Serialize};

pub mod account;

pub mod post;

pub trait TAggregate: Default + Serialize + DeserializeOwned + Sync + Send {
    type Command;
    type Event: std::cmp::PartialEq + std::fmt::Debug;
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
}
