mod account;
pub mod middleware;
mod post;
pub mod response;
use account::account_router;
use axum::{routing::get, Router};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api/v1/accounts", account_router())
        .nest("/api/v1/posts", post::post_router())
}
