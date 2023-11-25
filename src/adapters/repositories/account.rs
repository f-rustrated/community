use crate::{
    domains::account::{Account, AccountStatus},
    services::{account::repository::AccountRepository, responses::BaseError},
};
use async_trait::async_trait;

use super::{pool, SqlRepository};

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
            .fetch_one(pool())
            .await?
        )
    }

    async fn add(&self, account: &Account) -> Result<Account, BaseError> {
        Ok(sqlx::query_as!(Account,
        r#"
            INSERT INTO account (uuid, name, status, hashed_password)
            VALUES ($1, $2, $3, $4)
            RETURNING id, uuid, name, status AS "status!: AccountStatus", hashed_password, created_at, updated_at
        "#,
        account.uuid,
        &account.name,
        AccountStatus::Active as AccountStatus,
        &account.hashed_password)
            .fetch_one(pool())
            .await?
        )
    }

    async fn update(&self, account: &Account) -> Result<(), BaseError> {
        todo!()
    }
}

#[cfg(test)]
mod account_repository_test {
    #[tokio::test]
    async fn test_get() {}
}
