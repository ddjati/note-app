use std::sync::{atomic::Ordering, Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Error;
use tokio::sync::Mutex;

use crate::{model::*, AppState};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status" : "ok",
        "message" : MESSAGE
    });

    return Json(json_response);
}

pub async fn metrics_handler(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status" : "ok",
        "db_hit_counter" : app.db_hit_counter.load(Ordering::SeqCst)
    });

    return Json(json_response);
}

async fn get_note(id: &String, app: &Arc<AppState>) -> Result<NoteModel, Error> {
    // get using query macro
    let query_result = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes WHERE id = ?")
        .bind(id)
        .fetch_one(&app.db)
        .await;

    // check & response
    app.db_hit_counter.fetch_add(1, Ordering::SeqCst);
    match query_result {
        Ok(note) => {
            return Ok(note);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn to_ok_note_response(note: NoteModel) -> Json<serde_json::Value> {
    let note_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "note": to_note_response(&note)
        })
    });
    return Json(note_response);
}

pub async fn get_note_handler(
    Path(id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // get using query macro
    let query_result = get_note(&id, &app).await;

    // check & response
    match query_result {
        Ok(note) => {
            return Ok(to_ok_note_response(note));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

async fn get_cqrs_note(id: String, app: &Arc<AppState>) -> Result<NoteModel, Error> {
    let cached_note = app.note_cache.get(&id).await;
    if cached_note.is_some() {
        let note = cached_note.unwrap();
        return Ok(note);
    }
    // get using query macro
    let query_result = get_note(&id, &app).await;

    // check & response
    match query_result {
        Ok(note) => {
            app.note_cache.insert(id, note.clone()).await;
            return Ok(note);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

async fn get_note_cached(
    id: String,
    app: &Arc<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match get_cqrs_note(id.to_string(), &app).await {
        Ok(note) => {
            app.note_cache.insert(id, note.clone()).await;
            let note_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "note": to_note_response(&note)
                })
            });

            return Ok(Json(note_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn get_note_handler_cached(
    Path(id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    return get_note_cached(id, &app).await;
}

pub async fn get_note_handler_thunder(
    Path(id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cached_note = app.note_cache.get(&id).await;

    if let Some(note) = cached_note {
        return Ok(to_ok_note_response(note));
    }

    if app.mutex_map.read().await.get(&id).is_none() {
        //app.map_mutex.lock()
        let _map_mtx = app.map_mutex.lock().await;
        if app.mutex_map.read().await.get(&id).is_none() {
            //init mutex for note id
            //double check locking
            app.mutex_map
                .write()
                .await
                .insert(id.to_string(), Mutex::new(false));
        }
        //app.map_mutex.unlock()
    }

    let read_mtx_map = app.mutex_map.read().await;

    let mut lock = read_mtx_map.get(&id).unwrap().lock().await;
    if *lock {
        panic!("Lock is used in another Task/Thread");
    }
    *lock = true;
    let json = get_note_cached(id.to_string(), &app).await;
    *lock = false;
    return json;
}
