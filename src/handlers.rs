// src/handlers.rs

use axum::{Json, extract::State, http::StatusCode};
use uuid::Uuid;
use chrono::Utc;
use axum::http::HeaderMap;
use crate::{models::{NewUser}, auth::hash_password};
use crate::models::{Note, NewNote};
use crate::db::Db;
use crate::auth::{verify_password, generate_jwt};
use crate::models::LoginRequest;
use crate::models::User;
use crate::auth::verify_jwt;


pub async fn create_note(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(payload): Json<NewNote>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims = verify_jwt(token).ok_or((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    let user_id = claims.sub;

    let note_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let note = sqlx::query_as::<_, Note>(
        r#"
        INSERT INTO notes (id, title, content, created_at, user_id)
        VALUES (?, ?, ?, ?, ?)
        RETURNING *
        "#
    )
    .bind(&note_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&now)
    .bind(&user_id)
    .fetch_one(&db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn get_notes(
    State(db): State<Db>,
    headers: HeaderMap,
) -> Result<Json<Vec<Note>>, (StatusCode, String)> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims = verify_jwt(token).ok_or((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    let user_id = claims.sub;

    let notes = sqlx::query_as::<_, Note>(
        r#"
        SELECT * FROM notes WHERE user_id = ?
        "#
    )
    .bind(&user_id)
    .fetch_all(&db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(notes))
}


pub async fn delete_note(
    State(db): State<Db>,
    headers: HeaderMap,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // 🔐 Extract user ID from JWT
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims = verify_jwt(token).ok_or((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    let user_id = claims.sub;

    // 🔍 Fetch the note and check ownership
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?")
        .bind(&id)
        .fetch_optional(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let note = match note {
        Some(n) => {
            if n.user_id != user_id {
                return Err((StatusCode::UNAUTHORIZED, "Not authorized".to_string()));
            }
            n
        }
        None => return Err((StatusCode::NOT_FOUND, "Note not found".to_string())),
    };

    // ✅ Delete the note
    sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(&id)
        .execute(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}


pub async fn update_note(
    State(db): State<Db>,
    headers: HeaderMap,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<NewNote>,
) -> Result<Json<Note>, (StatusCode, String)> {
    // 🔐 Extract user ID from JWT
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims = verify_jwt(token).ok_or((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    let user_id = claims.sub;

    // 🔍 Fetch the note and check ownership
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?")
        .bind(&id)
        .fetch_optional(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let note = match note {
        Some(n) => {
            if n.user_id != user_id {
                return Err((StatusCode::UNAUTHORIZED, "Not authorized".to_string()));
            }
            n
        }
        None => return Err((StatusCode::NOT_FOUND, "Note not found".to_string())),
    };

    // ✅ Update the note
    let updated_note = sqlx::query_as::<_, Note>(
        "UPDATE notes SET title = ?, content = ? WHERE id = ? RETURNING *",
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&id)
    .fetch_one(&db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_note))
}

pub async fn register_user(
    State(db): State<Db>,
    Json(payload): Json<NewUser>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    // Hash the password
    let hashed = hash_password(&payload.password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hashing failed".into()))?;

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Insert into DB
    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, username, password_hash, created_at)
        VALUES (?, ?, ?, ?)
        "#,
        id,
        payload.username,
        hashed,
        now,
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => Ok((StatusCode::CREATED, "User registered".into())),
        Err(e) if e.to_string().contains("UNIQUE constraint") => {
            Err((StatusCode::CONFLICT, "Username already taken".into()))
        }
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to register".into())),
    }
}

pub async fn login_user(
    State(db): State<Db>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(&db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match user {
        Some(u) => u,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into())),
    };

    let valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    let token = generate_jwt(&user.id);
    Ok(Json(token))
}