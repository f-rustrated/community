pub mod auth;
pub mod post;
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::OnceLock;

use crate::services::cross_cutting_traits::TransactionUnitOfWork;
use crate::services::responses::BaseError;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct SqlRepository {
    pub pool: &'static PgPool,
    pub transaction: Option<sqlx::Transaction<'static, sqlx::Postgres>>,
}

impl SqlRepository {
    pub fn new(pool: &'static PgPool) -> Self {
        SqlRepository {
            pool: pool,
            transaction: None,
        }
    }

    pub fn get_conn(
        &mut self,
    ) -> Either<&'static PgPool, sqlx::Transaction<'static, sqlx::Postgres>> {
        match self.transaction.take() {
            Some(trx) => Either::Right(trx),
            None => Either::Left(self.pool),
        }
    }
}

#[async_trait]
impl TransactionUnitOfWork for SqlRepository {
    async fn begin(&mut self) -> Result<(), BaseError> {
        match self.transaction {
            None => {
                self.transaction = Some(
                    self.pool
                        .begin()
                        .await
                        .map_err(|_| BaseError::DatabaseConnectionFailed)?,
                )
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
                eprintln!("Transaction begun but failed for some reason {}", err);
                BaseError::TransactionError
            }),
        }
    }
    async fn rollback(&mut self) -> Result<(), BaseError> {
        //TODO you make it!
        todo!()
    }
}

pub fn pool() -> &'static PgPool {
    static SQLX_CONNECTION_POOL: OnceLock<PgPool> = OnceLock::new();

    SQLX_CONNECTION_POOL.get_or_init(|| {
        std::thread::spawn(|| {
            #[tokio::main]
            async fn _get_pool() -> PgPool {
                let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
                PgPoolOptions::new()
                    .max_connections(30)
                    .connect(&url)
                    .await
                    .unwrap()
            }

            _get_pool()
        })
        .join()
        .expect("Failed to run connection pool fetching operation!")
    })
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_transaction() {
        //TODO test transaction
    }
}
