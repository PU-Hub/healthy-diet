use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::sync::Arc;
use tracing::error;

use crate::model::ENVKey;
use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};

#[derive(Deserialize)]
pub struct GenerateTitleRequest {
    pub message: String,
}

#[derive(Deserialize)]
struct NodeTitleResponse {
    pub title: String,
}

pub async fn generate_room_title_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(room_id): Path<String>,
    Json(payload): Json<GenerateTitleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let client = Client::new();
    let agent_api_url =
        env::var(ENVKey::AGENT_API_URL).expect("[generate title] Missing AGENT_API_URL");
    let normalized_url = agent_api_url.trim_end_matches('/');

    let node_api_url = if normalized_url.ends_with("/api/chat") {
        format!(
            "{}/generate_title",
            normalized_url.trim_end_matches("/api/chat")
        )
    } else if normalized_url.ends_with("/chat") {
        format!(
            "{}/generate_title",
            normalized_url.trim_end_matches("/chat")
        )
    } else {
        format!("{normalized_url}/generate_title")
    };

    let res = client
        .post(node_api_url)
        .json(&json!({ "message": payload.message }))
        .send()
        .await
        .map_err(|e| {
            error!("call Node.js generate_title failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Generate title request failed".into(),
                }),
            )
        })?;

    let node_data: NodeTitleResponse = res.json().await.map_err(|e| {
        error!("parse generate_title response failed: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Invalid generate title response".into(),
            }),
        )
    })?;

    let new_title = node_data.title;

    sqlx::query(
        r#"
        UPDATE diet_chat_history
        SET title = $1
        WHERE room_id = $2 AND user_id = $3
        "#,
    )
    .bind(&new_title)
    .bind(&room_id)
    .bind(auth_user.user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("update diet_chat_history title failed: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update title".into(),
            }),
        )
    })?;

    if let Err(e) = sqlx::query(
        r#"
        UPDATE chat_rooms
        SET title = $1, updated_at = NOW()
        WHERE room_id = $2 AND user_id = $3
        "#,
    )
    .bind(&new_title)
    .bind(&room_id)
    .bind(auth_user.user_id)
    .execute(&state.db)
    .await
    {
        error!("update chat_rooms title failed: {:?}", e);
    }

    Ok((StatusCode::OK, Json(json!({ "title": new_title }))))
}
