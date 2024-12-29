mod handler;
mod model;
mod route;
mod schema;

use std::sync::Arc;

use dotenv::dotenv;
use route::{main_app, metrics_app};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::net::TcpListener;

pub struct AppState {
    db: MySqlPool,
}

async fn create_db_pool() -> sqlx::MySqlPool {
    let db_url = std::env::var("DATABASE_URL").expect("env var DATABASE_URL must set");
    let max_db_connections: u32 = std::env::var("MAX_DB_CONNECTIONS")
        .expect("env var MAX_DB_CONNECTIONS must set")
        .parse::<u32>()
        .expect("MAX_DB_CONNECTIONS expect u32");
    match MySqlPoolOptions::new()
        .max_connections(max_db_connections)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            return pool;
        }
        Err(err) => {
            println!(
                "❌ Failed to connect to the database {:?}: {:?}",
                db_url, err
            );
            std::process::exit(1);
        }
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("before spawn");
    let task_main_app = tokio::task::spawn(async {
        let pool = create_db_pool().await;
        let app = main_app(Arc::new(AppState { db: pool }));
        let addr = "0.0.0.0:8000";
        tracing::debug!("listening on {}", addr);

        axum::serve(
            TcpListener::bind(addr).await.unwrap(),
            app.into_make_service(),
        )
        .await
    });

    // The `/metrics` endpoint should not be publicly available. If behind a reverse proxy, this
    // can be achieved by rejecting requests to `/metrics`. In this example, a second server is
    // started on another port to expose `/metrics`.
    let task_metrics_app = tokio::task::spawn(async {
        let app = metrics_app();

        // NOTE: expose metrics endpoint on a different port
        let addr = "0.0.0.0:3001";
        tracing::debug!("listening on {}", addr);
        axum::serve(
            TcpListener::bind(addr).await.unwrap(),
            app.into_make_service(),
        )
        .await
    });
    println!("before join");
    let (_main_task, _metrics_task) = tokio::join!(task_main_app, task_metrics_app);
    // metrics_task.unwrap().unwrap();
    // main_task.unwrap().unwrap();

    println!("🌟 REST API Service 🌟");
}
