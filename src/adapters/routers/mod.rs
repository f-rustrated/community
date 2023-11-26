mod account;

use axum::{
    routing::{get},
    Router,
};
use crate::adapters::routers::account::account_router;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/accounts", account_router())
}
