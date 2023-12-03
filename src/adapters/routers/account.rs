// use axum::extract::Path;
// use axum::routing::{get, post};
// use axum::{extract, Router};

// use crate::adapters::repositories::{pool, SqlRepository};
// use crate::domains::account::commands::{CreateAccount, SignInAccount};
// use crate::services::account::handlers::AccountHandler;

// async fn account_handler() -> AccountHandler<SqlRepository> {
//     AccountHandler {
//         repo: SqlRepository {
//             pool: pool().await,
//             transaction: None,
//         },
//     }
// }

// pub async fn account_router() -> Router {
//     let get_account = move |Path(id): Path<i64>| async move {
//         match account_handler().await.get_account(id).await {
//             Ok(response) => {
//                 println!("response: {:?}", response);
//                 "ok".to_owned()
//             }
//             Err(_) => "error".to_owned(),
//         }
//     };

//     let sign_in_account = move |extract::Json(payload): extract::Json<SignInAccount>| async move {
//         match account_handler().await.sign_in_account(payload).await {
//             Ok(response) => {
//                 println!("response: {:?}", response);
//                 "ok".to_owned()
//             }
//             Err(_) => "error".to_owned(),
//         }
//     };

//     let sign_up_account = move || async move {
//         match account_handler()
//             .await
//             .sign_up_account(CreateAccount {
//                 email: "test".to_owned(),
//                 password: "test".to_owned(),
//             })
//             .await
//         {
//             Ok(response) => {
//                 println!("response: {:?}", response);
//                 "ok".to_owned()
//             }
//             Err(_) => "error".to_owned(),
//         }
//     };

//     Router::new()
//         .route("/:id", get(get_account))
//         .route("/sign-in", post(sign_in_account))
//         .route("/sign-up", post(sign_up_account))
// }
