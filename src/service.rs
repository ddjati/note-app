use std::collections::HashMap;

use moka::future::Cache;
use sqlx::MySqlPool;
use tokio::sync::{Mutex, RwLock};

use crate::model::NoteModel;

pub struct AppState {
    pub db: MySqlPool,
    pub note_cache: Cache<String, NoteModel>,
    pub mutex_map: RwLock<HashMap<String, Mutex<bool>>>,
}
