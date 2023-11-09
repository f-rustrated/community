use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use community::domain::{commands::CreateUser, models::User};

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}
