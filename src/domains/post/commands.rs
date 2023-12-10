//TODO stuff fields in the following commands!
pub struct CreatePost {}
pub struct DeletePost {
    pub id: i64,
}
pub struct UpvotePost {
    pub id: i64,
    pub user_id: String,
}

#[cfg(test)]
pub mod helper {
    use super::{CreatePost, DeletePost, UpvotePost};

    pub fn create_post() -> CreatePost {
        todo!()
    }

    pub fn delete_post() -> DeletePost {
        todo!()
    }

    pub fn upvote_post() -> UpvotePost {
        todo!()
    }
}
