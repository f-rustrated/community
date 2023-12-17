use crate::domains::post::commands::{DeletePost, UpdatePost, UpvotePost};
use crate::domains::post::{commands::CreatePost, CommunityPost};

use super::repository::{PostCommandRepository, PostQueryRepository};
use super::{
    super::{
        cross_cutting_traits::TransactionUnitOfWork,
        responses::{ApplicationResponse, ServiceError},
    },
    query::ListCommunityPosts,
};

#[derive(Clone)]
pub struct PostHandler<R> {
    pub repo: R,
}

impl<R> PostHandler<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

// 1. checking user context
// 2. creating post
impl<R: PostCommandRepository + TransactionUnitOfWork> PostHandler<R> {
    pub async fn create_post(
        &mut self,
        cmd: CreatePost,
    ) -> Result<ApplicationResponse, ServiceError> {
        self.repo.begin().await?;
        let aggregate = CommunityPost::new(cmd);
        let res = self.repo.add(&aggregate).await?;
        self.repo.commit().await?;
        Ok(ApplicationResponse::I64(res))
    }

    pub async fn update_post(&mut self, cmd: UpdatePost) -> Result<(), ServiceError> {
        self.repo.begin().await?;
        let mut aggregate = self.repo.get(cmd.id).await?;
        aggregate.update(cmd);
        self.repo.update(&aggregate).await?;
        self.repo.commit().await?;
        Ok(())
    }

    pub async fn delete_post(&mut self, cmd: DeletePost) -> Result<(), ServiceError> {
        self.repo.begin().await?;
        let mut aggregate = self.repo.get(cmd.id).await?;
        aggregate.delete();
        self.repo.update(&aggregate).await?;
        self.repo.commit().await?;
        Ok(())
    }

    pub async fn upvote_post(&mut self, cmd: UpvotePost) -> Result<(), ServiceError> {
        self.repo.begin().await?;
        let mut aggregate = self.repo.get(cmd.id).await?;
        aggregate.upvote(cmd)?;
        self.repo.update(&aggregate).await?;
        self.repo.commit().await?;
        Ok(())
    }
}

impl<R: PostQueryRepository> PostHandler<R> {
    pub async fn get_post(&self, id: i64) -> Result<CommunityPost, ServiceError> {
        Ok(self.repo.get(id).await?)
    }
    pub async fn list_posts(
        &self,
        query: ListCommunityPosts,
    ) -> Result<Vec<CommunityPost>, ServiceError> {
        Ok(self.repo.list_posts(&query).await?)
    }
}
