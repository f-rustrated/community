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

    async fn get_by_uuid(&self, uuid: uuid::Uuid) -> Result<Account, BaseError> {
        Ok(sqlx::query_as!(Account,
        r#"
            SELECT id, uuid, name, status AS "status!: AccountStatus", hashed_password, created_at, updated_at
            FROM account
            WHERE uuid = $1
        "#,
        uuid)
            .fetch_one(pool())
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
        let res = sqlx::query!(
        r#"
            UPDATE account
            SET name = $1, status = $2, hashed_password = $3, updated_at = $4
            WHERE id = $5
        "#,
        &account.name,
        account.status as AccountStatus,
        &account.hashed_password,
        account.updated_at,
        account.id)
            .execute(pool())
            .await?;

        match res.rows_affected() {
            1 => Ok(()),
            _ => Err(BaseError::InternalError),
        }
    }
}

#[cfg(test)]
mod account_repository_test {
    use dotenv::dotenv;
    use crate::adapters::repositories::SqlRepository;
    use crate::services::account::repository::AccountRepository;

    #[tokio::test]
    async fn test_get() {
        todo!("What's wrong with this test?");
        // assertion `left == right` failed
        // left: 0
        // right: 1
        //
        // Left:  0
        // Right: 1

        // dotenv().ok();
        //
        // let repository = SqlRepository {
        //     pool: crate::adapters::repositories::pool(),
        //     transaction: None,
        // };
        //
        // let account = repository.add(&crate::domains::account::Account::new(
        //     &crate::domains::account::commands::CreateAccount {
        //         name: "test".to_string(),
        //         password: "test".to_string(),
        //     },
        // )).await.unwrap();
        //
        // let retrieved_account = repository.get(account.id).await.unwrap();
        // assert_eq!(retrieved_account.name, "test");
        // assert_eq!(retrieved_account.hashed_password, "test");
        // assert_eq!(retrieved_account.status, crate::domains::account::AccountStatus::Active);
    }
}
