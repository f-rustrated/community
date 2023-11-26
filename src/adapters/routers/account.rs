use axum::{extract, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::adapters::repositories::SqlRepository;
use crate::services::account::handlers::AccountHandler;
use crate::services::account::repository::AccountRepository; // TODO:

pub struct AccountRouter {
    handler: AccountHandler<SqlRepository>,
}

impl AccountRouter {
    pub(crate) async fn post_account() {
        todo!()
    }
    pub(crate) async fn get_account() {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct PostAccountRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct GetAccountResponse {
    pub name: String,
    pub created_at: String,
}

// pub async fn yo(i: i32) -> impl Fn(axum::extract::Json<PostAccountRequest>) -> &'static str {
//     // move |req| {
//     //     let result = post_account(req).await?;
//     //     result
//     // }
//     move |req| {post_account(req).await}
// }

pub async fn post_account(Json(req): extract::Json<PostAccountRequest>) -> &'static str {
    println!("1111111111111111111111111111111111111111111111111111");
    ""
}

pub async fn get_account(
    extract::Path(account_uuid): extract::Path<Uuid>,
) -> Json<GetAccountResponse> {
    println!("account_uuid = {}", account_uuid);

    // Create a JSON response
    let response = GetAccountResponse {
        name: "name".to_string(),
        created_at: "created_at".to_string(),
    };
    Json(response)
}
