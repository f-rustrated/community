use axum::extract::Path;
use axum::routing::{get, post};
use axum::Router;

use crate::adapters::repositories::{pool, SqlRepository};
use crate::domains::account::commands::CreateAccount;
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
                println!("response: {:?}", response);
                "ok".to_owned()
            }
            Err(_) => "error".to_owned(),
        }
    };

    let create_account = move || async move {
        match account_handler()
            .create_account(CreateAccount {
                name: "test".to_owned(),
                password: "test".to_owned(),
            })
            .await
        {
            Ok(response) => {
                println!("response: {:?}", response);
                "ok".to_owned()
            }
            Err(_) => "error".to_owned(),
        }
    };

    Router::new()
        .route("/:id", get(get_account))
        .route("/", post(create_account))
}
