mod handler;
mod model;
mod route;
mod schema;

use std::sync::Arc;

use axum::http::{header::CONTENT_TYPE, Method};
use dotenv::dotenv;
use route::create_router;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let db_url = std::env::var("DATABASE_URL").expect("env var DATABASE_URL must set");
    let max_db_connections: u32 = std::env::var("MAX_DB_CONNECTIONS")
        .expect("env var MAX_DB_CONNECTIONS must set")
        .parse::<u32>()
        .expect("MAX_DB_CONNECTIONS expect u32");
    let pool = match MySqlPoolOptions::new()
        .max_connections(max_db_connections)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!(
                "❌ Failed to connect to the database {:?}: {:?}",
                db_url, err
            );
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    println!("✅ Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
