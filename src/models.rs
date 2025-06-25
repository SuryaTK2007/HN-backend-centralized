// src/models.rs

use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct NewNote {
    pub title: String,
    pub content: String,
    // No user_id here — we inject it from JWT in handler
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub user_id: String, 
}


#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String, // plain password input, to be hashed
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
