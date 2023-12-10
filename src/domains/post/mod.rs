pub mod commands;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::services::responses::ServiceError;

use self::commands::{CreatePost, UpvotePost};

#[derive(Serialize)]
pub struct CommunityPost {
    pub id: i64,
    account_id: i64,
    title: String,
    thumbnail: Option<String>,
    category: PostCategory,
    body: String,
    pub status: PostStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // TODO how are you going to make this struct as aggregate root?
}

impl CommunityPost {
    pub fn new(cmd: CreatePost) -> Self {
        unimplemented!()
    }

    pub fn delete(&mut self) {
        self.status = PostStatus::Deleted
    }
    pub fn upvote(&mut self, cmd: UpvotePost) -> Result<(), ServiceError> {
        unimplemented!()
    }
}

#[derive(Serialize)]
pub enum PostCategory {
    Default,
    Knowledge,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum PostStatus {
    Created,
    Deleted,
    Edited,
}
