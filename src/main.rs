// src/main.rs
mod models;
mod handlers;
mod rate_limiter;

use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use models::AppState;
use handlers::get_nfl_schedule;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let redis_client = redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to connect to Redis");

    let state = Arc::new(AppState {
        client: reqwest::Client::new(),
        api_key: std::env::var("RUNDOWN_API_KEY")
            .expect("RUNDOWN_API_KEY must be set"),
        redis: redis_client,
    });

    let app = Router::new()
        .route("/schedule", get(get_nfl_schedule))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Server running on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}