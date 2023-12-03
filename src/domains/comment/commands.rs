use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct addComment {
    pub account_id: i64,
    pub target_id: String,
    // post, comment
    pub target_type: char,
    pub message: String
}

#[derive(Deserialize, Clone)]
pub struct saveComment {
    pub account_id: i64,
    pub target_id: String,
    // post, comment
    pub target_type: char,
    pub message: String
}

#[derive(Deserialize, Clone)]
pub struct deleteComment {
    pub id: Bigserial
}