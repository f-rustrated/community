//TODO stuff fields in the following commands!
use serde::Deserialize;

// TODO implement request body separately in router?
#[derive(Deserialize, Clone)]
pub struct CreatePost {
    pub account_id: i64,
    pub title: String,
    pub thumbnail: Option<String>,
    pub category: String,
    pub body: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdatePost {
    pub id: i64,
    pub title: String,
    pub thumbnail: Option<String>,
    pub category: String,
    pub body: String,
}

#[derive(Deserialize, Clone)]
pub struct DeletePost {
    pub id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpvotePost {
    pub id: i64,
    pub user_id: String,
}

#[cfg(test)]
pub mod helper {
    use super::{CreatePost, DeletePost, UpvotePost};

    pub fn create_post() -> CreatePost {
        // CreatePost{ account_id: todo!(), title: todo!(), thumbnail: todo!(), category: todo!(), body: todo!() }
        todo!()
    }

    pub fn delete_post() -> DeletePost {
        todo!()
    }

    pub fn upvote_post() -> UpvotePost {
        todo!()
    }
}
