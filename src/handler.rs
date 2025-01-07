use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Error;

use crate::{model::*, schema::*, AppState};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status" : "ok",
        "message" : MESSAGE
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

pub async fn get_note_handler_cached(
    Path(id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cached_note = app.temp_map.get(&id);
    if cached_note.is_some() {
        let note = cached_note.unwrap();
        return Ok(to_ok_note_response(note));
    }
    // get using query macro
    let query_result = get_note(&id, &app).await;

    // check & response
    match query_result {
        Ok(note) => {
            app.temp_map
                .insert(id, note.clone(), Duration::from_millis(1));
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

pub async fn get_note_handler_thunder(
    Path(id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cached_note = app.temp_map.get(&id);
    
    if cached_note.is_some() {
        let note = cached_note.unwrap();
        return Ok(to_ok_note_response(note));
    }
    // get using query macro
    let query_result = get_note(&id, &app).await;

    // check & response
    match query_result {
        Ok(note) => {
            app.temp_map
                .insert(id, note.clone(), Duration::from_millis(1));
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

pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let notes = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes ORDER BY id LIMIT ? OFFSET ?")
        .bind(limit as i32)
        .bind(offset as i32)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status" : "error",
                "message" : format!("Database error: {}",e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    // Response
    let note_responses = notes
        .iter()
        .map(|note| to_note_response(&note))
        .collect::<Vec<NoteModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": note_responses.len(),
        "notes": note_responses
    });

    Ok(Json(json_response))
}

pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO notes (id, title, content) VALUES (?, ?, ?)"#)
        .bind(&id)
        .bind(&body.title)
        .bind(&body.content)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    // Duplicate err check
    match query_result {
        Err(err) => {
            if err.contains("Duplicate entry") {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": "Note already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }

            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
        _ => (), // OK
    };

    // Get inserted note by ID
    let note = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes WHERE id = ?")
        .bind(&id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let note_response = serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
                "note": to_note_response(&note)
        })
    });

    return Ok(Json(note_response));
}

pub async fn edit_note_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // validate note with query macro
    let query_result = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes WHERE id = ?")
        .bind(&id)
        .fetch_one(&data.db)
        .await;

    // fetch the result
    let note = match query_result {
        Ok(note) => note,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            ));
        }
    };

    // parse data
    let is_published = body.is_published.unwrap_or(note.is_published != 0);
    let i8_is_published = is_published as i8;

    // Update (if empty, use old value)
    let update_result =
        sqlx::query(r#"UPDATE notes SET title = ?, content = ?, is_published = ? WHERE id = ?"#)
            .bind(&body.title.unwrap_or_else(|| note.title))
            .bind(&body.content.unwrap_or_else(|| note.content))
            .bind(i8_is_published)
            .bind(&id)
            .execute(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}", e)
                    })),
                )
            })?;

    // if no data affected (or deleted when wanted to update)
    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    } else {
        // DO NOTHING
    }

    // get updated data
    let updated_note = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes WHERE id = ?")
        .bind(&id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let note_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "note": to_note_response(&updated_note)
        })
    });

    return Ok(Json(note_response));
}

pub async fn delete_note_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // delete with query macro
    let query_result = sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(&id)
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;

    // response
    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::OK)
}
