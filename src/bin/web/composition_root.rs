use community::domains::account::{CreateAccount, SignInAccount};
use community::domains::post::commands::{CreatePost, DeletePost, UpdatePost};
use community::domains::post::CommunityPost;
use community::services::post::handler::PostHandler;
use community::services::post::query::ListCommunityPosts;
use community::{
    adapters::repositories::SqlRepository,
    services::{
        account::handlers::AccountHandler,
        responses::{ApplicationResponse, ServiceError},
    },
};

pub struct CompositionRoot<T>(pub T);
impl CompositionRoot<CreateAccount> {
    pub async fn sign_up_account(self) -> Result<ApplicationResponse, ServiceError> {
        let mut handler = AccountHandler::new(SqlRepository::new().await);
        handler.sign_up_account(self.0).await
    }
}

impl CompositionRoot<SignInAccount> {
    pub async fn sign_in_account(self) -> Result<ApplicationResponse, ServiceError> {
        let handler = AccountHandler::new(SqlRepository::new().await);
        handler.sign_in_account(self.0).await
    }
}

impl CompositionRoot<CreatePost> {
    pub async fn create_post(self) -> Result<ApplicationResponse, ServiceError> {
        let mut handler = PostHandler::new(SqlRepository::new().await);
        // TODO(retrieve account_id from token)
        handler.create_post(self.0).await
    }
}

impl CompositionRoot<UpdatePost> {
    pub async fn update_post(self) -> Result<(), ServiceError> {
        let mut handler = PostHandler::new(SqlRepository::new().await);
        // TODO(validate account)
        handler.update_post(self.0).await
    }
}

impl CompositionRoot<DeletePost> {
    pub async fn delete_post(self) -> Result<(), ServiceError> {
        let mut handler = PostHandler::new(SqlRepository::new().await);
        // TODO(validate account)
        handler.delete_post(self.0).await
    }
}

impl CompositionRoot<ListCommunityPosts> {
    pub async fn list_posts(self) -> Result<Vec<CommunityPost>, ServiceError> {
        let handler = PostHandler::new(SqlRepository::new().await);
        handler.list_posts(self.0).await
    }
}

impl CompositionRoot<i64> {
    pub async fn get_post(self) -> Result<CommunityPost, ServiceError> {
        let handler = PostHandler::new(SqlRepository::new().await);
        handler.get_post(self.0).await
    }
}

impl<T> From<T> for CompositionRoot<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
