use axum::{Json, http::StatusCode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use tracing::error;

use crate::{api::model::ErrorResponse, model::ENVKey, utils::jwt::AuthUser};

#[derive(Deserialize, Serialize)]
pub struct AgentApproveRequest {
    pub thread_id: String,
    pub action: String, // approve or reject
}

pub async fn approve_agent(
    _: AuthUser,
    Json(payload): Json<AgentApproveRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let client = Client::new();

    let node_api_url = format!(
        "{}/approve",
        env::var(ENVKey::AGENT_API_URL).map_err(|e| {
            error!("cannot get env value {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Server error".to_string(),
                }),
            )
        })?
    );

    let response = client
        .post(node_api_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            error!("AI server connect error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "AI server connect error".to_string(),
                }),
            );
        })?;

    let json_data: serde_json::Value = response.json().await.unwrap_or_default();

    Ok(Json(json_data))
}
