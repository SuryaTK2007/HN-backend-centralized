// src/routes.rs

use axum::{Router, routing::{post, delete}};
use crate::handlers::{create_note, get_notes, delete_note, update_note};
use crate::db::Db;
use crate::handlers::{register_user, login_user};

pub fn create_routes(db: Db) -> Router {
    Router::new()
        .route("/notes", post(create_note).get(get_notes))
        .route("/notes/:id", delete(delete_note).put(update_note)) 
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .with_state(db)
}
