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

pub async fn delete_note(
    State(db): State<Db>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let query = "DELETE FROM notes WHERE id = ?";

    let result = sqlx::query(query)
        .bind(&id)
        .execute(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Note not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_note(
    State(db): State<Db>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<NewNote>,
) -> Result<Json<Note>, (StatusCode, String)> {
    let query = "UPDATE notes SET title = ?, content = ? WHERE id = ?";

    let result = sqlx::query(query)
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(&id)
        .execute(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Note not found".into()));
    }

    // Return the updated note
    let updated_note = Note {
        id,
        title: payload.title,
        content: payload.content,
        created_at: chrono::Utc::now().to_rfc3339(), // Not fetched again, just regenerated
    };

    Ok(Json(updated_note))
}
