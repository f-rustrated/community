use async_trait::async_trait;

use crate::{domains::post::CommunityPost, services::responses::BaseError};

use super::query::ListPosts;

#[async_trait]
pub trait PostCommandRepository {
    async fn get(&self, id: i64) -> Result<CommunityPost, BaseError>;

    async fn add(&mut self, aggregate: &CommunityPost) -> Result<i64, BaseError>;

    async fn update(&mut self, aggregate: &CommunityPost) -> Result<(), BaseError>;
}

#[async_trait]
pub trait PostQueryRepository {
    async fn get(&self, id: i64) -> Result<CommunityPost, BaseError>;
    async fn list_posts(&self, query: &ListPosts) -> Result<Vec<CommunityPost>, BaseError>;
}
