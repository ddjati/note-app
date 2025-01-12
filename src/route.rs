use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handler::{
        get_note_handler, get_note_handler_cached, get_note_handler_thunder, health_check_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/api/notes/:id", get(get_note_handler))
        .route("/api/cached_notes/:id", get(get_note_handler_cached))
        .route("/api/thunder_notes/:id", get(get_note_handler_thunder))
        .with_state(app_state)
}
