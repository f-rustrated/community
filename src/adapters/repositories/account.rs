use crate::{
    domains::account::{Account, AccountEvent, AccountStatus},
    services::{account::repository::AccountRepository, responses::BaseError},
};
use async_trait::async_trait;

use super::SqlRepository;

#[async_trait]
impl AccountRepository for SqlRepository {
    async fn get(&self, id: i64) -> Result<Account, BaseError> {
        Ok(sqlx::query_as!(Account,
        r#"
            SELECT id, uuid, name, status AS "status!: AccountStatus", hashed_password, created_at, updated_at
            FROM account
            WHERE id = $1
        "#,
        id)
            .fetch_one(self.pool)
            .await?
        )
    }

    async fn get_by_email(&self, email: String) -> Result<Account, BaseError> {
        Ok(sqlx::query_as!(Account,
        r#"
            SELECT id, uuid, name, status AS "status!: AccountStatus", hashed_password, created_at, updated_at
            FROM account
            WHERE name = $1
        "#,
        email)
            .fetch_one(self.pool)
            .await?
        )
    }

    async fn add(&mut self, account: &[AccountEvent]) -> Result<i64, BaseError> {
        todo!()
        // let rec = sqlx::query!(
        //     r#"
        //     INSERT INTO account (uuid, name, status, hashed_password)
        //     VALUES ($1, $2, $3, $4)
        //     RETURNING id
        // "#,
        //     account.uuid,
        //     &account.name,
        //     AccountStatus::Active as AccountStatus,
        //     &account.hashed_password
        // )
        // .fetch_one(self.transaction()?)
        // .await?;

        // Ok(rec.id)
    }

    async fn update(&mut self, account: &Account) -> Result<(), BaseError> {
        todo!()
    }
}

#[cfg(test)]
mod account_repository_test {
    use crate::adapters::repositories::pool;

    async fn set_up() {
        sqlx::query!(
            r#"
            TRUNCATE 
                post,
                account
            CASCADE;
        "#
        )
        .execute(pool().await)
        .await
        .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_add() {
        '_given: {
            todo!()
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get() {
        '_given: {
            todo!()
        }
    }
}
