use std::net::SocketAddr;

use crate::routers::router;
mod routers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
