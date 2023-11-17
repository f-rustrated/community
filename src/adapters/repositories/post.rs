use async_trait::async_trait;

use crate::domains::models::{repositories::CommunityPostRepository, CommunityPost};

use super::SqlRepository;

#[async_trait]
impl CommunityPostRepository for SqlRepository {
    async fn get(&self, id: i64) -> CommunityPost {
        todo!();
    }

    async fn add(&self, post: CommunityPost) {
        todo!();
    }

    async fn update(&self, post: CommunityPost) {
        todo!();
    }
}
