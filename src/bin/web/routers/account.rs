use axum::routing::post;
use axum::{Json, Router};
use community::domains::account::commands::{CreateAccount, SignInAccount};
use community::services::responses::ApplicationResponse;

use crate::composition_root::CompositionRoot;
use crate::routers::response::AxumResponse;

use super::response::AxumError;

#[axum::debug_handler]
pub async fn sign_in_account(
    Json(payload): Json<SignInAccount>,
) -> Result<AxumResponse<ApplicationResponse>, AxumError> {
    Ok(AxumResponse(
        CompositionRoot(payload).sign_in_account().await?,
    ))
}

#[axum::debug_handler]
pub async fn sign_up_account(
    Json(payload): Json<CreateAccount>,
) -> Result<AxumResponse<ApplicationResponse>, AxumError> {
    Ok(AxumResponse(
        CompositionRoot(payload).sign_up_account().await?,
    ))
}

pub fn account_router() -> Router {
    Router::new()
        .route("/sign-in", post(sign_in_account))
        .route("/sign-up", post(sign_up_account))
}
