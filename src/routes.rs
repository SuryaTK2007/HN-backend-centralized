// src/routes.rs

use axum::{Router, routing::{get, post, delete, put}};
use crate::handlers::{create_note, get_notes, delete_note, update_note};
use crate::db::Db;

pub fn create_routes(db: Db) -> Router {
    Router::new()
        .route("/notes", post(create_note).get(get_notes))
        .route("/notes/:id", delete(delete_note).put(update_note)) // 🆕 Add this line
        .with_state(db)
}
