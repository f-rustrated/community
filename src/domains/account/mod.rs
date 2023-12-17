use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod response;
use bcrypt;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::services::responses::ServiceError;

use self::commands::CreateAccount;

pub mod commands;
#[cfg(test)]
pub mod test;

// TODO: decide salt mechanism later
const SALT: [u8; 16] = [
    49, 129, 3, 11, 159, 22, 1, 194, 94, 245, 142, 24, 21, 91, 99, 19,
];
const JWT_LIFETIME: u64 = 86400; // seconds
const JWT_SECRET: &str = "TODO";

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Deleted,
    Abnormal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub(crate) id: i64,
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) status: AccountStatus,
    pub(crate) hashed_password: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl Account {
    pub(crate) fn new(cmd: &CreateAccount) -> Result<Self, ServiceError> {
        let hashed_password = Self::create_password(&cmd.password)?;

        Ok(Self {
            id: Default::default(),
            uuid: Uuid::new_v4(),
            name: cmd.email.to_string(),
            status: AccountStatus::Active,
            hashed_password,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // * For the given aggregate, verify passed password
    pub(crate) fn verify_password(&self, plain_password: &str) -> Result<(), ServiceError> {
        let is_verified = bcrypt::verify(plain_password, &self.hashed_password).map_err(|err| {
            eprintln!("bcrypt verify failed: {}", err);
            ServiceError::HashLibError(err.to_string())
        })?;
        if !is_verified {
            return Err(ServiceError::AuthenticationError(
                "password doesn't match".to_string(),
            ));
        }
        Ok(())
    }

    // * static method to create hashed_password
    pub(crate) fn create_password(plaintext_password: &str) -> Result<String, ServiceError> {
        match bcrypt::hash_with_salt(plaintext_password, bcrypt::DEFAULT_COST, SALT) {
            Ok(hash_parts) => Ok(hash_parts.to_string()),
            Err(err) => {
                eprintln!("bcrypt hash failed: {}", err);
                Err(ServiceError::HashLibError(err.to_string()))
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
        let encoding_key = EncodingKey::from_secret(JWT_SECRET.as_bytes());

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
