pub mod account;
pub mod post;
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgConnection, PgPool};
use std::sync::OnceLock;

use crate::services::cross_cutting_traits::TransactionUnitOfWork;
use crate::services::responses::BaseError;

pub struct SqlRepository {
    pub(crate) pool: &'static PgPool,
    pub(crate) transaction: Option<sqlx::Transaction<'static, sqlx::Postgres>>,
}
impl SqlRepository {
    pub async fn new() -> Self {
        Self {
            pool: pool().await,
            transaction: Default::default(),
        }
    }
}

impl SqlRepository {
    pub fn transaction(&mut self) -> Result<&mut PgConnection, BaseError> {
        match self.transaction.as_mut() {
            Some(trx) => Ok(&mut *trx),
            None => {
                tracing::error!("Transaction has not begun!");
                Err(BaseError::TransactionError)
            }
        }
    }
}

#[async_trait]
impl TransactionUnitOfWork for SqlRepository {
    async fn begin(&mut self) -> Result<(), BaseError> {
        match self.transaction {
            None => {
                self.transaction = Some(self.pool.begin().await?);
            }
            Some(_) => Err(BaseError::TransactionError)?,
        }
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), BaseError> {
        match self.transaction.take() {
            None => {
                eprintln!("Transaction hasn't begun!");
                Err(BaseError::TransactionError)?
            }
            Some(trx) => trx.commit().await.map_err(|err| {
                eprintln!("Transaction begun but failed to commit: {}", err);
                BaseError::TransactionError
            }),
        }
    }
    async fn rollback(&mut self) -> Result<(), BaseError> {
        match self.transaction.take() {
            None => {
                eprintln!("Transaction hasn't begun!");
                Err(BaseError::TransactionError)
            }
            Some(trx) => trx.rollback().await.map_err(|err| {
                eprintln!("Transaction begun but failed to roll back: {}", err);
                BaseError::TransactionError
            }),
        }
    }
}

pub async fn pool() -> &'static PgPool {
    static SQLX_CONNECTION_POOL: OnceLock<PgPool> = OnceLock::new();
    if SQLX_CONNECTION_POOL.get().is_none() {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

        let _ = SQLX_CONNECTION_POOL.set(
            PgPoolOptions::new()
                .max_connections(30)
                .connect(&url)
                .await
                .unwrap(),
        );
    }
    SQLX_CONNECTION_POOL.get().unwrap()
}

impl From<sqlx::Error> for BaseError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!("Database Error! {}", value);
        Self::DatabaseError
    }
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_transaction() {
        //TODO test transaction
    }
}
