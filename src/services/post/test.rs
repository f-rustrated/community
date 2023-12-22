#[cfg(test)]
pub mod post_handler {

    use crate::{
        adapters::repositories::SqlRepository,
        domains::post::{
            commands::helper::{create_post, delete_post, upvote_post},
            PostStatus,
        },
        services::{
            post::{handler::PostHandler, repository::PostCommandRepository},
            responses::ApplicationResponse,
        },
    };

    #[tokio::test]
    async fn test_create_post() {
        '_given: {
            dotenv::dotenv().ok();

            '_when: {
                let cmd = create_post();
                let mut handler = PostHandler {
                    repo: SqlRepository::new().await,
                };
                let Ok(ApplicationResponse::I64(post_id)) = handler.create_post(cmd).await else {
                    panic!("Must not fail!")
                };

                '_then: {
                    let repo = SqlRepository::new().await;
                    let _aggregate = repo.get(post_id).await.unwrap();
                }
            }
        }
    }

    #[tokio::test]
    async fn test_delete_post() {
        '_given: {
            dotenv::dotenv().ok();

            let mut handler = PostHandler {
                repo: SqlRepository::new().await,
            };
            let Ok(ApplicationResponse::I64(post_id)) = handler.create_post(create_post()).await
            else {
                panic!("Must not fail in preparation!")
            };

            '_when: {
                let cmd = delete_post();
                let Ok(()) = handler.delete_post(cmd).await else {
                    panic!("Must not fail on test subeject")
                };
                '_then: {
                    let repo = SqlRepository::new().await;
                    let aggregate = repo.get(post_id).await.unwrap();
                    assert_eq!(aggregate.status, PostStatus::Deleted);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_upvote_post() {
        '_given: {
            dotenv::dotenv().ok();

            let mut handler = PostHandler {
                repo: SqlRepository::new().await,
            };
            let Ok(ApplicationResponse::I64(post_id)) = handler.create_post(create_post()).await
            else {
                panic!("Must not fail in preparation!")
            };

            '_when: {
                let cmd = upvote_post();
                let Ok(()) = handler.upvote_post(cmd).await else {
                    panic!("Must not fail on test subeject")
                };
                '_then: {
                    let repo = SqlRepository::new().await;
                    let _aggregate = repo.get(post_id).await.unwrap();

                    // TODO check upvote number increased by 1
                }
            }
        }
    }
}
