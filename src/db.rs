// src/db.rs

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;

pub type Db = SqlitePool;

pub async fn init_pool() -> Result<Db, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
