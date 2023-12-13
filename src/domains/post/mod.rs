pub mod commands;

use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::domains::post::commands::UpdatePost;

use crate::services::responses::ServiceError;

use self::commands::{CreatePost, UpvotePost};

#[derive(Serialize)]
pub struct CommunityPost {
    pub id: i64,
    pub(crate) account_id: i64,
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
        Self {
            id: 0,
            account_id: cmd.account_id,
            title: cmd.title,
            thumbnail: cmd.thumbnail,
            category: PostCategory::Default,
            body: cmd.body,
            status: PostStatus::Created,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update(&mut self, cmd: UpdatePost) {
        self.title = cmd.title;
        self.thumbnail = cmd.thumbnail;
        self.category = PostCategory::Default;
        self.body = cmd.body;
        self.updated_at = Utc::now();
        self.updated_at = Utc::now();
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
