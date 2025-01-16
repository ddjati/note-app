use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Error;
use tokio::{sync::Mutex, time::Instant};

use crate::{model::*, service::*};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status" : "ok",
        "message" : MESSAGE
    });

    return Json(json_response);
}

async fn get_note_from_db(id: &String, app: &Arc<AppState>) -> Result<(NoteModel, Metrics), Error> {
    let start = Instant::now();
    // get using query macro
    let query_result = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes WHERE id = ?")
        .bind(id)
        .fetch_one(&app.db)
        .await;

    let mut mtr = Metrics {
        is_from_db: Some(true),
        ..Default::default()
    };

    mtr.db_duration = Some(start.elapsed().as_micros());

    // check & response
    match query_result {
        Ok(note) => {
            return Ok((note, mtr));
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn to_ok_note_response(note: NoteModel, metrics: Metrics) -> Json<serde_json::Value> {
    let mut note_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "note": to_note_response(&note)
        }),
    });
    if metrics != Metrics::default() {
        if let Some(resp) = note_response.as_object_mut() {
            resp.insert("metrics".to_string(), serde_json::json!(metrics));
        }
    }
    return Json(note_response);
}

fn err_not_found(id: String) -> (StatusCode, axum::Json<serde_json::Value>) {
    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Note with ID: {} not found", id)
    });
    return (StatusCode::NOT_FOUND, Json(error_response));
}

fn err_internal_server(e: Error) -> (StatusCode, axum::Json<serde_json::Value>) {
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"status": "error","message": format!("{:?}", e)})),
    );
}

pub async fn get_note_handler(
    Path(id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // get using query macro
    let query_result = get_note_from_db(&id, &app).await;

    // check & response
    match query_result {
        Ok((note, mtr)) => {
            return Ok(to_ok_note_response(note, mtr));
        }
        Err(sqlx::Error::RowNotFound) => {
            return Err(err_not_found(id));
        }
        Err(e) => {
            return Err(err_internal_server(e));
        }
    }
}

async fn get_cqrs_note(id: String, app: &Arc<AppState>) -> Result<(NoteModel, Metrics), Error> {
    let cached_note = app.note_cache.get(&id).await;
    if cached_note.is_some() {
        let note = cached_note.unwrap();
        let mt = Metrics {
            ..Default::default()
        };
        return Ok((note, mt));
    }
    // get using query macro
    let query_result = get_note_from_db(&id, &app).await;

    // check & response
    match query_result {
        Ok((note, mt)) => {
            app.note_cache.insert(id, note.clone()).await;
            return Ok((note, mt));
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
        Ok((note, mt)) => {
            return Ok(to_ok_note_response(note, mt));
        }
        Err(sqlx::Error::RowNotFound) => {
            return Err(err_not_found(id));
        }
        Err(e) => {
            return Err(err_internal_server(e));
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
    //get cached note
    let cached_note = app.note_cache.get(&id).await;

    if let Some(note) = cached_note {
        return Ok(to_ok_note_response(note, Metrics::default()));
    }

    // check mutex for id
    if app.mutex_map.read().await.get(&id).is_none() {
        //init mutex for id
        let mut mtx_map = app.mutex_map.write().await;
        if mtx_map.get(&id).is_none() {
            //init mutex for note id
            //double check locking
            mtx_map.insert(id.to_string(), Mutex::new(false));
        }
    }

    let cqrs_result: Result<(NoteModel, Metrics), Error>;
    let read_mtx_map = app.mutex_map.read().await;

    {
        let mut lock = read_mtx_map.get(&id).unwrap().lock().await;
        if *lock {
            panic!("Lock is used in another Task/Thread");
        }
        *lock = true;
        cqrs_result = get_cqrs_note(id.to_string(), &app).await;
        *lock = false;
    }

    match cqrs_result {
        Ok((note, mt)) => {
            return Ok(to_ok_note_response(note, mt));
        }
        Err(sqlx::Error::RowNotFound) => {
            return Err(err_not_found(id));
        }
        Err(e) => {
            return Err(err_internal_server(e));
        }
    }
}
