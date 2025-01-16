use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::get,
    Router,
};
use moka::future::Cache;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    handler::{
        get_note_handler, get_note_handler_cached, get_note_handler_thunder, health_check_handler,
    },
    model::NoteModel,
    service::AppState,
};

pub async fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app_state = Arc::new(AppState {
        db: get_my_sql_pool().await,
        note_cache: get_cache().await,
        mutex_map: RwLock::new(HashMap::new()),
    });
    return Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/api/notes/:id", get(get_note_handler))
        .route("/api/cached_notes/:id", get(get_note_handler_cached))
        .route("/api/thunder_notes/:id", get(get_note_handler_thunder))
        .with_state(app_state)
        .layer(cors);
}

async fn get_cache() -> Cache<String, NoteModel> {
    let cache_ttl_millis: u64 = std::env::var("CACHE_TTL_MILLIS")
        .expect("env var CACHE_TTL_MILLIS must set")
        .parse::<u64>()
        .expect("CACHE_TTL_MILLIS expect u64");
    println!("CACHE_TTL_MILLIS = {}", cache_ttl_millis);
    let cache_ttl_millis: u64 = std::env::var("CACHE_TTL_MILLIS")
        .expect("env var CACHE_TTL_MILLIS must set")
        .parse::<u64>()
        .expect("CACHE_TTL_MILLIS expect u64");
    return Cache::builder()
        .time_to_live(Duration::from_millis(cache_ttl_millis))
        .build();
}

async fn get_my_sql_pool() -> MySqlPool {
    let db_url = std::env::var("DATABASE_URL").expect("env var DATABASE_URL must set");
    let max_db_connections: u32 = std::env::var("MAX_DB_CONNECTIONS")
        .expect("env var MAX_DB_CONNECTIONS must set")
        .parse::<u32>()
        .expect("MAX_DB_CONNECTIONS expect u32");
    println!("MAX_DB_CONNECTIONS = {}", max_db_connections);
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
            tracing::error!(
                "❌ Failed to connect to the database {:?}: {:?}",
                db_url,
                err
            );
            std::process::exit(1);
        }
    };
}
