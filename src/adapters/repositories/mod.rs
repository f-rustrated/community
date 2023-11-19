pub mod auth;
pub mod post;

use crate::domains::account::Account;
use sqlx::postgres::PgPoolOptions;

// TODO implement the following repository using sqlx.
pub struct SqlRepository {
    pool: sqlx::PgPool,
}

impl SqlRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(SqlRepository { pool })
    }

    pub async fn add_account(&self, account: &Account) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO account (id, uuid, name, hashed_password) VALUES (?, ?, ?, ?)",
            account.id,
            account.uuid,
            account.name
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_account(&self, id: i32) -> Result<Account, sqlx::Error> {
        // need FrowRow
        let query = sqlx::query_as::<_, Account>("SELECT * FROM account WHERE id = ?").bind(id);

        // Execute the query and return the result
        let result = query.fetch_all(&self.pool).await?;
        Ok(result)
    }
}
