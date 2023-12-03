use crate::{
    domains::comment::Comment,
    services::{comment::repository::CommentRepository, responses::BaseError},
};


use async_trait::async_trait;

use super::{pool, SqlRepository};

#[aysnc_trait]
impl CommentRepository for SqlRepository {
    async fn insert(&self, comment: &Comment) -> Result<Comment, BaseError> {
        Ok(sqlx::query_as!(Comment,
            r#"
                INSERT INTO comment (accounet_id, target_id, target_type, message,edited_yn,deleted_yn,created_at,update_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, accounet_id, target_id, target_type, message, edited_yn, deleted_yn, created_at,updated_at
            "#,
            &comment.accounet_id,
            &comment.target_id,
            &comment.target_type,
            &comment.message,
            &comment.edited_yn,
            &comment.deleted_yn,
            &comment.created_at,
            &comment.update_at)
                .fetch_one(pool())
                .await?
        )
    }
}