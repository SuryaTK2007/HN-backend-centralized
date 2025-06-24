// src/models.rs

use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNote {
    pub title: String,
    pub content: String,
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