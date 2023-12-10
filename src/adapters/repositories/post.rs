use async_trait::async_trait;

use crate::{
    domains::post::CommunityPost,
    services::{post::repository::PostCommandRepository, responses::BaseError},
};

use super::SqlRepository;

#[async_trait]
impl PostCommandRepository for SqlRepository {
    async fn get(&self, id: i64) -> Result<CommunityPost, BaseError> {
        unimplemented!()
    }

    async fn add(&mut self, aggregate: &CommunityPost) -> Result<i64, BaseError> {
        unimplemented!()
    }

    async fn update(&mut self, aggregate: &CommunityPost) -> Result<(), BaseError> {
        unimplemented!()
    }
}
