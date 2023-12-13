pub mod commands;
pub mod response;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domains::post::commands::UpdatePost;

use crate::services::responses::ServiceError;

use self::commands::{CreatePost, UpvotePost};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "post_category", rename_all = "lowercase")]
pub enum PostCategory {
    Default,
    Knowledge,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[sqlx(type_name = "post_status", rename_all = "lowercase")]
pub enum PostStatus {
    Created,
    Deleted,
    Edited,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityPost {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) title: String,
    pub(crate) thumbnail: Option<String>,
    pub(crate) category: PostCategory,
    pub(crate) body: String,
    pub(crate) status: PostStatus,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    // TODO how are you going to make this struct as aggregate root?
}

impl CommunityPost {
    pub fn new(cmd: CreatePost) -> Self {
        Self {
            id: 0,
            account_id: cmd.account_id.unwrap(),
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
        self.status = PostStatus::Edited;
        self.updated_at = Utc::now();
    }

    pub fn delete(&mut self) {
        self.status = PostStatus::Deleted;
        self.updated_at = Utc::now();
    }
    pub fn upvote(&mut self, cmd: UpvotePost) -> Result<(), ServiceError> {
        unimplemented!()
    }
}

