use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

use community::adapters::routers::router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL not present");

    // let pool = pool();
    // let repository = SqlRepository::new(pool);

    let app = router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
