mod routers;
use dotenv::dotenv;
use std::net::SocketAddr;

use crate::routers::router;
pub mod composition_root;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // TODO The following will be replaced by database-related state?
    dotenv().ok();

    let app = router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
