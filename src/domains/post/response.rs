use serde::Serialize;
use crate::domains::post::CommunityPost;

#[derive(Serialize)]
pub enum CommunityPostResponse {
    Post(CommunityPost),
}

impl From<CommunityPost> for CommunityPostResponse {
    fn from(value: CommunityPost) -> Self {
        Self::Post(value)
    }
}