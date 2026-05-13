use axum::{Json, http::StatusCode};
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Gemma4HealthResponse {
    pub target_url: String,
    pub model: String,
    pub running: bool,
    pub http_status_code: Option<u16>,
    pub message: String,
}

pub async fn gemma4_health_handler() -> (StatusCode, Json<Gemma4HealthResponse>) {
    let target_url = "http://127.0.0.1:8080";
    let client = Client::new();

    match client
        .get(target_url)
        .timeout(Duration::from_secs(3))
        .send()
        .await
    {
        Ok(response) => (
            StatusCode::OK,
            Json(Gemma4HealthResponse {
                target_url: target_url.to_string(),
                model: "gemma4".to_string(),
                running: true,
                http_status_code: Some(response.status().as_u16()),
                message: "gemma4 service is reachable on localhost:8080".to_string(),
            }),
        ),
        Err(error) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(Gemma4HealthResponse {
                target_url: target_url.to_string(),
                model: "gemma4".to_string(),
                running: false,
                http_status_code: None,
                message: format!("cannot reach gemma4 service on localhost:8080: {}", error),
            }),
        ),
    }
}
