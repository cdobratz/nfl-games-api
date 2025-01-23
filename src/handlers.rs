// src/handlers.rs
use axum::{extract::State, http::HeaderMap, http::StatusCode, Json};
use std::sync::Arc;
use crate::models::{AppState, Schedule, RateLimitError};
use crate::rate_limiter::check_rate_limits;

const NFL_SPORT_ID: &str = "2";
const API_BASE_URL: &str = "https://api.apilayer.com/therundown";

pub async fn get_nfl_schedule(
    State(state): State<Arc<AppState>>,
) -> Result<(HeaderMap, Json<Schedule>), (StatusCode, Json<RateLimitError>)> {
    check_rate_limits(&state.redis).await?;

    let url = format!(
        "{}/sports/{}/schedule",
        API_BASE_URL,
        NFL_SPORT_ID
    );
    
    let response = state.client
        .get(&url)
        .header("apikey", &state.api_key)
        .send()
        .await
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RateLimitError {
                message: "Failed to fetch data from upstream API".to_string(),
            }),
        ))?;

    let mut headers = HeaderMap::new();
    if let Some(monthly_limit) = response.headers().get("x-ratelimit-limit-month") {
        headers.insert("x-ratelimit-limit-month", monthly_limit.clone());
    }
    if let Some(monthly_remaining) = response.headers().get("x-ratelimit-remaining-month") {
        headers.insert("x-ratelimit-remaining-month", monthly_remaining.clone());
    }
    if let Some(daily_limit) = response.headers().get("x-ratelimit-limit-day") {
        headers.insert("x-ratelimit-limit-day", daily_limit.clone());
    }
    if let Some(daily_remaining) = response.headers().get("x-ratelimit-remaining-day") {
        headers.insert("x-ratelimit-remaining-day", daily_remaining.clone());
    }

    let schedule = response
        .json::<Schedule>()
        .await
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RateLimitError {
                message: "Failed to parse API response".to_string(),
            }),
        ))?;

    Ok((headers, Json(schedule)))
}