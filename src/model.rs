use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Clone)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// For json response
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct Metrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_db: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_duration: Option<u128>, // duration in microsec
}

//Convert DB Model to Response
pub fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    return NoteModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        is_published: note.is_published != 0,
        created_at: note.created_at.unwrap(),
        updated_at: note.updated_at.unwrap(),
        content: note.content.to_owned(),
    };
}
