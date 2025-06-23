use axum::serve;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

mod db;
mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::init_pool().await.expect("Failed to connect to DB");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    let app = routes::create_routes(db)
        .layer(ServiceBuilder::new().layer(cors));

    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running on http://{}", addr);
    serve(listener, app.into_make_service()).await.unwrap();
}