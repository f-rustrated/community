use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ListCommunityPosts {
    pub(crate) from: i64,
    pub(crate) size: i64,
}
