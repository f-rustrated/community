use axum::{
    extract,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

pub(crate) mod account;
use crate::domains::account::{
    commands::{CreateAccount, SignInAccount},
    Account,
};
use crate::services::account::{handlers::AccountHandler, repository::AccountRepository};
use account::{GetAccountResponse, PostAccountRequest};

use crate::adapters::repositories::{pool, SqlRepository};

pub async fn root() -> &'static str {
    "Hello, World!"
}

fn account_handler() -> AccountHandler<SqlRepository> {
    // FIXME:
    let pool = pool();
    let repository = SqlRepository::new(pool);
    AccountHandler::new(repository)
}

pub fn router() -> Router {
    // TODO: axum doesn't allow any pre-initialized instances inside its handler

    let post_account_handler = move |extract::Json(req): extract::Json<PostAccountRequest>| async move {
        println!("1111111111111111111111111111111111111111111111111111");
        let cmd = CreateAccount {
            account_name: req.name,
            password: req.password,
        };
        match account_handler().create_account(cmd).await {
            Ok(resp) => "ok".to_owned(),
            Err(_) => "error".to_owned(),
        };
        "123"
    };

    let get_account_handler = move |extract::Path(account_uuid): extract::Path<Uuid>| async move {
        let mut account: Option<Account> = None;
        match account_handler().get_account(account_uuid).await {
            // Ok(resp) => (resp),
            Ok(acc) => account = Some(acc),
            Err(err) => {
                println!("get account failed: {:?}", err)
            }
        };

        match account {
            Some(acc) => {
                let response = GetAccountResponse {
                    name: acc.name,
                    created_at: acc.created_at.to_string(),
                };
                axum::Json(response)
            }
            None => {
                println!("No reference available.");
                let response = GetAccountResponse {
                    name: "error".to_string(),
                    created_at: "error".to_string(),
                };
                axum::Json(response)
            }
        }
    };

    Router::new()
        .route("/", get(root))
        .route("/accounts", post(post_account_handler))
        .route("/accounts/:account_uuid", get(get_account_handler))
    // .route("/asdf", get(post_account_handler))
}
