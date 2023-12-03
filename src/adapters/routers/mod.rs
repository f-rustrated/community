mod account;

// use crate::adapters::routers::account::account_router;
use axum::{routing::get, Router};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn router() -> Router {
    Router::new().route("/", get(root))
    // .nest("/accounts", account_router())
}
