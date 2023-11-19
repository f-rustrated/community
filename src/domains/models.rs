use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct CommunityPost {
    id: i64,
    account_id: i64,
    title: String,
    thumbnail: Option<String>,
    category: PostCategory,
    body: String,
    status: PostStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub enum PostCategory {
    Default,
    Knowledge,
}

#[derive(Serialize)]
pub enum PostStatus {
    Created,
    Deleted,
    Edited,
}
