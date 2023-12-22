use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
pub mod response;
use bcrypt;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
pub mod enums;
use crate::{config::config, services::responses::ServiceError};
pub use enums::*;
pub mod events;
pub use events::*;
pub mod commands;
pub use commands::*;
pub mod errors;
pub use errors::*;

use super::TAggregate;

#[cfg(test)]
pub mod test;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Account {
    pub(crate) id: i64,
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) status: AccountStatus,
    pub(crate) hashed_password: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl TAggregate for Account {
    type Command = AccountCommand;
    type Event = AccountEvent;
    type Error = AccountError;
    fn handle(&mut self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            Self::Command::CreateAccount(create_account) => {
                Self::validate_password(&create_account.password)?;
                self.create_account(create_account)
            }
            Self::Command::SignInAccount(sign_in_account) => {
                self.verify_password(&sign_in_account.password)?;
                Ok(vec![Self::Event::SignedIn {
                    email: sign_in_account.email,
                    password: sign_in_account.password,
                }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Self::Event::AccountCreated { .. } => todo!(),
            Self::Event::SignedIn { .. } => todo!(),
        }
    }
}

impl Account {
    fn create_account(&mut self, cmd: CreateAccount) -> Result<Vec<AccountEvent>, AccountError> {
        self.id = Default::default();
        self.uuid = Uuid::new_v4();
        self.name = cmd.email.to_string();
        self.status = AccountStatus::Active;
        self.hashed_password = Self::create_password(&cmd.password)?;
        self.created_at = Utc::now();
        self.updated_at = Utc::now();

        Ok(vec![AccountEvent::AccountCreated {
            name: cmd.email,
            hashed_password: self.hashed_password.clone(),
            id: self.id,
            uuid: self.uuid,
            created_at: self.created_at,
        }])
    }

    // * For the given aggregate, verify passed password
    pub(crate) fn verify_password(&self, plain_password: &str) -> Result<(), AccountError> {
        let is_verified = bcrypt::verify(plain_password, &self.hashed_password).map_err(|err| {
            eprintln!("bcrypt verify failed: {}", err);
            AccountError::HashLibError(err.to_string())
        })?;
        if !is_verified {
            return Err(AccountError::AuthenticationError(
                "password doesn't match".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_password(given_password: &str) -> Result<(), AccountError> {
        const MIN_PASSWORD_LEN: usize = 8;
        if given_password.len() < MIN_PASSWORD_LEN {
            return Err(PasswordPolicy::NotEnoughChars)?;
        }

        // TODO: compile regex once and reuse
        let pattern = Regex::new(r#"[a-z]"#).unwrap();
        if !pattern.is_match(given_password) {
            return Err(PasswordPolicy::AtLeastOneLower)?;
        }

        let pattern = Regex::new(r#"[A-Z]"#).unwrap();
        if !pattern.is_match(given_password) {
            return Err(PasswordPolicy::AtLeastOneUpper)?;
        }

        let pattern = Regex::new(r#"\d"#).unwrap();
        if !pattern.is_match(given_password) {
            return Err(PasswordPolicy::AtLeastOneNumber)?;
        }

        let pattern = Regex::new(r#"[!@%^&*?_-]"#).unwrap();
        if !pattern.is_match(given_password) {
            return Err(PasswordPolicy::AtLeastOneSpecialChar)?;
        }

        Ok(())
    }

    // * static method to create hashed_password
    pub(crate) fn create_password(plaintext_password: &str) -> Result<String, AccountError> {
        match bcrypt::hash_with_salt(plaintext_password, bcrypt::DEFAULT_COST, SALT) {
            Ok(hash_parts) => Ok(hash_parts.to_string()),
            Err(err) => {
                eprintln!("bcrypt hash failed: {}", err);
                Err(AccountError::HashLibError(err.to_string()))
            }
        }
    }

    pub(crate) fn create_access_token(&self) -> Result<String, ServiceError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let claim = JWTClaim {
            sub: self.uuid.to_string(),
            nbf: now,
            exp: now + JWT_LIFETIME,
        };
        let header = Header::default(); // default HS256
        let encoding_key = EncodingKey::from_secret(config().jwt_secret.as_bytes());

        encode(&header, &claim, &encoding_key).map_err(|err| {
            tracing::error!("jwt encode failed: {}", err);
            ServiceError::JWTError(err.to_string())
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JWTClaim {
    pub sub: String,
    pub nbf: u64,
    pub exp: u64,
}

// TODO: decide salt mechanism later
const SALT: [u8; 16] = [
    49, 129, 3, 11, 159, 22, 1, 194, 94, 245, 142, 24, 21, 91, 99, 19,
];
const JWT_LIFETIME: u64 = 86400; // seconds
