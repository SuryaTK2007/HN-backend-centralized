// src/routes.rs

use axum::{Router, routing::post};
use crate::handlers::{create_note, get_notes};
use crate::db::Db;

pub fn create_routes(db: Db) -> Router {
    Router::new()
        .route("/notes", post(create_note).get(get_notes))
        .with_state(db)
}
