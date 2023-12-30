use serde_json::json;

use crate::{
    domains::{
        account::{Account, AccountEvent, AccountStatus},
        TEvent,
    },
    services::{account::repository::AccountRepository, responses::BaseError},
};

use super::SqlRepository;

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

    async fn add(&mut self, events: &[AccountEvent]) -> Result<i64, BaseError> {
        // TODO need for improvement as it the following could be simplified more into one that invokes async call once.
        // TODO need to develope `event enveloper`
        for event in events {
            let _ = sqlx::query!(
                r#"
                INSERT INTO events (
                    aggregate_type ,
                    aggregate_id   ,
                    sequence       ,
                    event_type     ,
                    event_version  ,
                    payload        ,
                    metadata          
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                "Account",
                "1",
                0,
                event.event_type(),
                event.event_version(),
                json!(event),
                serde_json::Value::Object(serde_json::Map::new())
            )
            .execute(self.pool)
            .await;
        }
        Ok(1)
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
