use axum::extract::Path;
use axum::routing::{get, post, put};
use axum::{extract, Router};

use crate::adapters::repositories::{pool, SqlRepository};
use crate::domains::account::commands::{SignUpAccount, SignInAccount, UpdateAccount};
use crate::services::account::handlers::AccountHandler;

fn account_handler() -> AccountHandler<SqlRepository> {
    AccountHandler {
        repo: SqlRepository {
            pool: pool(),
            transaction: None,
        },
    }
}

pub fn account_router() -> Router {
    let get_account = move |Path(id): Path<i64>| async move {
        match account_handler().get_account(id).await {
            Ok(response) => {
                "ok".to_owned()
            }
            Err(_) => "error".to_owned(),
        }
    };

    let sign_in_account = move |extract::Json(payload): extract::Json<SignInAccount>| async move {
        match account_handler()
            .sign_in_account(payload)
            .await {
            Ok(response) => {
                response
            }
            Err(_) => "error".to_owned(),
        }
    };

    let sign_up_account =
        move |extract::Json(payload): extract::Json<SignUpAccount>| async move {
        match account_handler()
            .sign_up_account(payload)
            .await {
            Ok(response) => {
                "ok".to_owned()
            }
            Err(_) => "error".to_owned(),
        }
    };

    let update_account =
        move |extract::Json(payload): extract::Json<UpdateAccount>| async move {
            match account_handler()
                .update_account(payload)
                .await {
                Ok(response) => {
                    "ok".to_owned()
                }
                Err(_) => "error".to_owned(),
            }
        };

    Router::new()
        .route("/:id", get(get_account))
        .route("/sign-in", post(sign_in_account))
        .route("/sign-up", post(sign_up_account))
        .route("/", put(update_account))
}
