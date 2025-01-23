// src/rate_limiter.rs
use axum::{http::StatusCode, Json};
use redis::AsyncCommands;
use crate::models::RateLimitError;

pub const DAILY_LIMIT: i32 = 50;
pub const MONTHLY_LIMIT: i32 = 750;

pub async fn check_rate_limits(
    redis: &redis::Client,
) -> Result<(), (StatusCode, Json<RateLimitError>)> {
    let mut conn = redis.get_async_connection().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RateLimitError {
                message: "Redis connection failed".to_string(),
            }),
        )
    })?;

    let daily_key = format!("api:daily:{}", chrono::Utc::now().date_naive());
    let monthly_key = format!("api:monthly:{}", chrono::Utc::now().format("%Y-%m"));

    let daily_count: i32 = conn.get(&daily_key).await.unwrap_or(0);
    let monthly_count: i32 = conn.get(&monthly_key).await.unwrap_or(0);

    if daily_count >= DAILY_LIMIT || monthly_count >= MONTHLY_LIMIT {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(RateLimitError {
                message: "You have exceeded your daily/monthly API rate limit".to_string(),
            }),
        ));
    }

    // Increment counters and set expiration
    let _: () = conn.incr(&daily_key, 1).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RateLimitError {
                message: "Failed to update rate limit counter".to_string(),
            }),
        )
    })?;

    let _: () = conn.incr(&monthly_key, 1).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RateLimitError {
                message: "Failed to update rate limit counter".to_string(),
            }),
        )
    })?;

    // Set expiration
    let _: () = conn.expire(&daily_key, 86400).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RateLimitError {
                message: "Failed to set expiration".to_string(),
            }),
        )
    })?;

    let _: () = conn.expire(&monthly_key, 2592000).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RateLimitError {
                message: "Failed to set expiration".to_string(),
            }),
        )
    })?;

    Ok(())
}