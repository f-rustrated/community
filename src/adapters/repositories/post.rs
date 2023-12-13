use async_trait::async_trait;

use crate::{
    domains::post::{CommunityPost, PostCategory, PostStatus},
    services::{post::repository::PostCommandRepository, responses::BaseError},
};
use crate::services::post::query::ListCommunityPosts;
use crate::services::post::repository::PostQueryRepository;

use super::SqlRepository;

#[async_trait]
impl PostCommandRepository for SqlRepository {
    async fn get(&self, id: i64) -> Result<CommunityPost, BaseError> {
        Ok(sqlx::query_as!(CommunityPost,
        r#"
            SELECT id, account_id, title, thumbnail, category AS "category!: PostCategory", body, status AS "status!: PostStatus", created_at, updated_at
            FROM post
            WHERE id = $1
            FOR UPDATE
        "#,
        id)
            .fetch_one(self.pool)
            .await?
        )
    }

    async fn add(&mut self, aggregate: &CommunityPost) -> Result<i64, BaseError> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO post (account_id, title, thumbnail, category, body, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            aggregate.account_id,
            &aggregate.title,
            aggregate.thumbnail,
            PostCategory::Default as PostCategory,
            &aggregate.body,
            PostStatus::Created as PostStatus
        )
            .fetch_one(self.transaction()?)
            .await?;
        Ok(rec.id)
    }

    async fn update(&mut self, aggregate: &CommunityPost) -> Result<(), BaseError> {
        sqlx::query!(
            r#"
            UPDATE post
            SET title = $1, thumbnail = $2, category = $3, body = $4, status = $5, updated_at = $6
            WHERE id = $7
            "#,
            &aggregate.title,
            aggregate.thumbnail,
            PostCategory::Default as PostCategory,
            &aggregate.body,
            aggregate.status.clone() as PostStatus,
            aggregate.updated_at,
            aggregate.id
        )
            .execute(self.transaction()?)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl PostQueryRepository for SqlRepository {
    async fn get(&self, id: i64) -> Result<CommunityPost, BaseError> {
        Ok(sqlx::query_as!(CommunityPost,
        r#"
            SELECT id, account_id, title, thumbnail, category AS "category!: PostCategory", body, status AS "status!: PostStatus", created_at, updated_at
            FROM post
            WHERE id = $1
        "#,
        id)
            .fetch_one(self.pool)
            .await?
        )
    }

    async fn list_posts(&self, query: &ListCommunityPosts) -> Result<Vec<CommunityPost>, BaseError> {
        Ok(sqlx::query_as!(CommunityPost,
        r#"
            SELECT id, account_id, title, thumbnail, category AS "category!: PostCategory", body, status AS "status!: PostStatus", created_at, updated_at
            FROM post
            WHERE id < $1
            ORDER BY id DESC
            LIMIT $2
        "#,
        query.from, query.size)
            .fetch_all(self.pool)
            .await?
        )
    }
}