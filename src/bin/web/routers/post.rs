use axum::{Json, Router};
use axum::extract::{Path, Query};

use community::domains::post::commands::{CreatePost, DeletePost, UpdatePost};
use community::domains::post::CommunityPost;
use community::services::post::query::ListCommunityPosts;
use community::services::responses::ApplicationResponse;

use crate::composition_root::CompositionRoot;
use crate::routers::response::AxumResponse;

use super::response::AxumError;

#[axum::debug_handler]
pub async fn create_post(
    Json(payload): Json<CreatePost>,
) -> Result<AxumResponse<ApplicationResponse>, AxumError> {
    Ok(AxumResponse(
        CompositionRoot(payload).create_post().await?,
    ))
}

#[axum::debug_handler]
pub async fn update_post(
    Json(payload): Json<UpdatePost>
) -> Result<AxumResponse<()>, AxumError> {
    Ok(AxumResponse(
        CompositionRoot(payload).update_post().await?,
    ))
}

#[axum::debug_handler]
pub async fn delete_post(
    Path(payload): Path<DeletePost>
) -> Result<AxumResponse<()>, AxumError> {
    Ok(AxumResponse(
        CompositionRoot(payload).delete_post().await?,
    ))
}

#[axum::debug_handler]
pub async fn get_post(
    Path(id): Path<i64>
) -> Result<AxumResponse<CommunityPost>, AxumError> {
    Ok(AxumResponse(
        CompositionRoot(id).get_post().await?,
    ))
}

#[axum::debug_handler]
pub async fn list_posts(
    Query(payload): Query<ListCommunityPosts>
) -> Result<AxumResponse<Vec<CommunityPost>>, AxumError> {
    Ok(AxumResponse(
        CompositionRoot(payload).list_posts().await?,
    ))
}

pub fn post_router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_post))
        .route("/", axum::routing::put(update_post))
        .route("/:id", axum::routing::delete(delete_post))
        .route("/:id", axum::routing::get(get_post))
        .route("/", axum::routing::get(list_posts))
}