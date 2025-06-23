// src/handlers.rs

use axum::{Json, extract::State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::models::{Note, NewNote};
use crate::db::Db;

pub async fn create_note(
    State(db): State<Db>,
    Json(payload): Json<NewNote>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    let note = Note {
        id: Uuid::new_v4().to_string(),
        title: payload.title,
        content: payload.content,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let query = "INSERT INTO notes (id, title, content, created_at) VALUES (?, ?, ?, ?)";

    sqlx::query(query)
        .bind(&note.id)
        .bind(&note.title)
        .bind(&note.content)
        .bind(&note.created_at)
        .execute(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn get_notes(
    State(db): State<Db>,
) -> Result<Json<Vec<Note>>, (StatusCode, String)> {
    let query = "SELECT * FROM notes ORDER BY created_at DESC";

    let notes = sqlx::query_as::<_, Note>(query)
        .fetch_all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(notes))
}
