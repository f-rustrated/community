use axum::{
    routing::{get},
    Router,
};

pub async fn root() -> &'static str {
    "Hello, World!"
}
pub fn router() -> Router {
    Router::new().route("/", get(root))
}
