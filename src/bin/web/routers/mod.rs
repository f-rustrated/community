use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use community::domains::{commands::CreateAccount, models::Account};

// pub async fn create_account(Json(payload): Json<CreateAccount>) -> (StatusCode, Json<Account>) {
//     let account = Account {
//         id: 1337,
//         accountname: payload.accountname,
//     };

//     (StatusCode::CREATED, Json(account))
// }

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn router() -> Router {
    Router::new().route("/", get(root))
    // .route("/accounts", post(create_account))
}
