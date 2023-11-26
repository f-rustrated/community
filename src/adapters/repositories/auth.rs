use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domains::account::{Account, AccountStatus},
    services::{account::repository::AccountRepository, responses::BaseError},
};

use super::{Either, SqlRepository};

#[async_trait]
impl AccountRepository for SqlRepository {
    async fn get(&self, id: i64) -> Result<Account, BaseError> {
        todo!()
    }

    async fn get_by_uuid(&mut self, uuid: Uuid) -> Result<Account, BaseError> {
        let query = sqlx::query_as!(
            Account,
            r#"
                SELECT
                    id, uuid, name, status AS "status: AccountStatus", hashed_password, created_at, updated_at
                FROM
                    account
                WHERE
                    uuid = $1
            "#,
            uuid
        );

        println!("1");

        let conn = self.get_conn();
        println!("2");
        let mut account: Option<Account> = None;
        let mut err: Option<BaseError> = None;
        match conn {
            Either::Left(pool) => {
                println!("3");
                match query.fetch_one(pool).await {
                    Ok(acc) => {
                        account = Some(acc);
                    }
                    Err(_) => err = Some(BaseError::DatabaseConnectionFailed),
                }
            }
            Either::Right(trx) => {
                // match query.fetch_one(trx).await {
                //     Ok(acc) => {
                //         account = Some(acc);
                //     }
                //     Err(e) => err = Some(BaseError::DatabaseConnectionFailed),
                // }
            }
        }

        Ok(account.unwrap())
    }

    async fn add(&self, account: &Account) -> Result<(), BaseError> {
        todo!()
    }

    async fn update(&self, account: &Account) -> Result<(), BaseError> {
        todo!()
    }
}
