use std::net::SocketAddr;
use std::sync::Arc;
use axum::handler::Handler;
use axum::Router;
use axum::routing::{get, post};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use community::adapters::routers::{root, account::create_account};

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = create_router(Arc::new(AppState { db: pool.clone() }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new().route("/", get(root))
        .route("/account", post(create_account))
        .with_state(app_state)
}
