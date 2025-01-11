mod handler;
mod model;
mod route;
mod schema;

use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};

use axum::http::{header::CONTENT_TYPE, Method};
use dotenv::dotenv;
use model::NoteModel;
use moka::future::Cache;
use route::create_router;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock},
};
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    db: MySqlPool,
    note_cache: Cache<String, NoteModel>,
    mutex_map: RwLock<HashMap<String, Mutex<bool>>>,
    map_mutex: Mutex<bool>,
    db_hit_counter: AtomicUsize,
}

fn _main() {
    let note = NoteModel {
        id: "anu".to_string(),
        title: "judul".to_string(),
        content: "isi".to_string(),
        ..Default::default()
    };
    let json_str = serde_json::to_string(&note).unwrap();
    tracing::debug!("{}", json_str);
    let note: NoteModel = serde_json::from_str(&json_str).unwrap();
    tracing::debug!("{:?}", note);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing::info!("🌟 REST API Service 🌟");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let pool = get_my_sql_pool().await;
    let app_state = Arc::new(AppState {
        db: pool.clone(),
        note_cache: Cache::builder()
            .time_to_live(Duration::from_millis(2))
            .build(),
        map_mutex: Mutex::new(false),
        mutex_map: RwLock::new(HashMap::new()),
        db_hit_counter: AtomicUsize::new(0),
    });
    let app = create_router(app_state).layer(cors);

    tracing::info!("✅ Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn get_my_sql_pool() -> MySqlPool {
    let db_url = std::env::var("DATABASE_URL").expect("env var DATABASE_URL must set");
    let max_db_connections: u32 = std::env::var("MAX_DB_CONNECTIONS")
        .expect("env var MAX_DB_CONNECTIONS must set")
        .parse::<u32>()
        .expect("MAX_DB_CONNECTIONS expect u32");
    tracing::debug!("MAX_DB_CONNECTIONS = {}", max_db_connections);
    match MySqlPoolOptions::new()
        .max_connections(max_db_connections)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            tracing::debug!("✅ Connection to the database is successful!");
            return pool;
        }
        Err(err) => {
            tracing::error!(
                "❌ Failed to connect to the database {:?}: {:?}",
                db_url,
                err
            );
            std::process::exit(1);
        }
    };
}
