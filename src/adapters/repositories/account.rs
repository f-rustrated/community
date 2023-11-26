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

    async fn add(&mut self, account: &Account) -> Result<Account, BaseError> {
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
            .fetch_one(  self.transaction()?)
            .await?
        )
    }

    async fn update(&mut self, account: &Account) -> Result<(), BaseError> {
        todo!()
    }
}

#[cfg(test)]
mod account_repository_test {
    use crate::adapters::repositories::{pool, SqlRepository};
    use crate::domains::account::test::account_create_helper;
    use crate::domains::account::AccountStatus;
    use crate::services::account::repository::AccountRepository;
    use crate::services::cross_cutting_traits::TransactionUnitOfWork;

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
    async fn test_add() {
        '_given: {
            dotenv::dotenv().ok();
            set_up().await;
            let test_password = "test".to_string();
            let test_email = "test@mail.com".to_string();
            let mut repository = SqlRepository {
                pool: crate::adapters::repositories::pool().await,
                transaction: None,
            };
            let account = account_create_helper(test_password.clone(), test_email.clone());

            '_when: {
                repository.begin().await.expect("begin fail!");
                repository.add(&account).await.expect("Shouldn't fail!");
                repository.commit().await.expect("begin fail!");

                let record = sqlx::query!(
                    r#"
                    SELECT id, name, status AS "status!: AccountStatus", hashed_password
                    FROM account
                "#,
                )
                .fetch_one(repository.pool)
                .await
                .unwrap();

                '_then: {
                    assert_eq!(record.name, "test@mail.com");
                    assert_eq!(record.hashed_password, "hashed_password");
                    assert_eq!(
                        record.status,
                        crate::domains::account::AccountStatus::Active
                    );
                    // * id is auto-generated value
                    assert_ne!(record.id, account.id)
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get() {
        '_given: {
            dotenv::dotenv().ok();
            set_up().await;

            let mut repository = SqlRepository {
                pool: crate::adapters::repositories::pool().await,
                transaction: None,
            };
            let test_password = "test".to_string();
            let test_email = "test@mail.com".to_string();

            repository.begin().await.expect("begin fail!");

            let account = repository
                .add(&account_create_helper(
                    test_password.clone(),
                    test_email.clone(),
                ))
                .await
                .expect("something happened at add!");

            repository.commit().await.unwrap();

            '_when: {
                let retrieved_account = repository
                    .get(account.id)
                    .await
                    .expect("something happened at get!");

                '_then: {
                    assert_eq!(retrieved_account.name, "test@mail.com");
                    assert_eq!(retrieved_account.hashed_password, "hashed_password");
                    assert_eq!(
                        retrieved_account.status,
                        crate::domains::account::AccountStatus::Active
                    );
                }
            }
        }
    }
}
