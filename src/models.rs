// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub api_key: String,
    pub redis: redis::Client,
}

#[derive(Debug, Serialize)]
pub struct RateLimitError {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct Schedule {
    pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub event_id: String,
    pub sport_id: i32,
    pub event_date: String,
    pub teams: Teams,
    pub lines: Option<Lines>,
}

#[derive(Deserialize, Serialize)]
pub struct Teams {
    pub home: Team,
    pub away: Team,
}

#[derive(Deserialize, Serialize)]
pub struct Team {
    pub team_id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Lines {
    pub spread: Option<Spread>,
    pub moneyline: Option<Moneyline>,
}

#[derive(Deserialize, Serialize)]
pub struct Spread {
    pub point_spread_home: f64,
    pub point_spread_away: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Moneyline {
    pub moneyline_home: i32,
    pub moneyline_away: i32,
}