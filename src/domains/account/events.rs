use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domains::TEvent;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountEvent {
    AccountCreated {
        id: i64,
        name: String,
        hashed_password: String,
        uuid: Uuid,
        created_at: DateTime<Utc>,
    },
    SignedIn {
        email: String,
        password: String,
    },
}

impl TEvent for AccountEvent {
    fn event_type(&self) -> String {
        let event_type_in_str = match self {
            Self::AccountCreated { .. } => "AccountCreated",
            Self::SignedIn { .. } => "SignIn",
        };
        event_type_in_str.to_string()
    }
    fn event_version(&self) -> String {
        "0.1".to_string()
    }
}
