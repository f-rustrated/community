pub mod account;
pub mod post;

use crate::domains::{AggregateContext, TAggregate, TEvent, TEventStore};
use crate::services::cross_cutting_traits::TransactionUnitOfWork;
use crate::services::responses::BaseError;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgConnection, PgPool};

use std::sync::OnceLock;

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

impl<A: TAggregate> TEventStore<A> for SqlRepository {
    type AC = AggregateContext<A>;

    async fn load_events(&self, aggregate_id: &str) -> Result<Vec<A::Event>, A::Error> {
        todo!()
    }

    async fn commit(
        &self,
        events: Vec<<A as TAggregate>::Event>,
        context: Self::AC,
    ) -> Result<(), A::Error> {
        if events.is_empty() {
            return Ok(());
        };
        let aggregate_id = context.aggregate_id;
        let mut current_sequence = context.current_sequence;

        for event in events {
            current_sequence += 1;
            let _ = sqlx::query!(
                r#"
                INSERT INTO events (
                    aggregate_type ,
                    aggregate_id   ,
                    sequence       ,
                    event_type     ,
                    event_version  ,
                    payload        
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                event.aggregate_type(),
                aggregate_id,
                current_sequence as i64,
                event.event_type(),
                event.event_version(),
                json!(event)
            )
            .execute(self.pool)
            .await;
        }
        Ok(())
    }
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
