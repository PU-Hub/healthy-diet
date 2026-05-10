use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::sse::{Event, Sse},
};
use base64::{Engine as _, engine::general_purpose};
use futures::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{convert::Infallible, time::Duration};
use tokio::fs;
use tracing::error;
use uuid::Uuid;

use crate::{
    api::model::ErrorResponse,
    model::{AppState, ENVKey},
    utils::jwt::AuthUser,
};

#[derive(Deserialize)]
pub struct AgentChatRequest {
    pub message: String,
    pub room_id: Option<Uuid>,
    pub user_context: Option<serde_json::Value>,
    pub image: Option<String>,
}

#[derive(Serialize)]
struct NodeAgentPayload {
    pub message: String,
    pub thread_id: String,
    pub chat_history_id: String,
    pub user_id: String,
    pub user_context: Option<serde_json::Value>,
    pub image: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyChatCheckResponse {
    pub ping_url: String,
    pub ping_ok: bool,
    pub proxy_chat_available: bool,
    pub ping_status_code: Option<u16>,
    pub ping_response: Option<String>,
}

pub async fn proxy_chat_check_handler() -> (StatusCode, Json<ProxyChatCheckResponse>) {
    let ping_url = "http://localhist:8001/ping";
    let client = Client::new();

    match client
        .get(ping_url)
        .timeout(Duration::from_secs(5))
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            let ping_ok = status.is_success();
            let ping_status_code = Some(status.as_u16());
            let ping_response = response.text().await.ok();

            let response_status = if ping_ok {
                StatusCode::OK
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            };

            (
                response_status,
                Json(ProxyChatCheckResponse {
                    ping_url: ping_url.to_string(),
                    ping_ok,
                    proxy_chat_available: ping_ok,
                    ping_status_code,
                    ping_response,
                }),
            )
        }
        Err(e) => {
            error!("proxy_chat_check ping failed: {:?}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ProxyChatCheckResponse {
                    ping_url: ping_url.to_string(),
                    ping_ok: false,
                    proxy_chat_available: false,
                    ping_status_code: None,
                    ping_response: Some(format!("ping request error: {}", e)),
                }),
            )
        }
    }
}

pub async fn proxy_agent_chat_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(request): Json<AgentChatRequest>,
) -> Result<
    Sse<impl futures::stream::Stream<Item = Result<Event, Infallible>>>,
    (StatusCode, Json<ErrorResponse>),
> {
    let client = Client::new();

    let target_room_id = request.room_id.unwrap_or_else(Uuid::new_v4);
    let original_message = request.message.clone();
    let original_image = request.image.clone();

    let saved_image_path =
        save_chat_image_if_needed(&auth_user.user_id, &target_room_id, original_image)
            .await
            .map_err(|e| {
                error!("save chat image failed: {:?}", e);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Image format invalid or cannot be saved".to_string(),
                    }),
                )
            })?;

    let chat_history_id = persist_user_chat_record(
        &state,
        auth_user.user_id,
        target_room_id,
        &original_message,
        saved_image_path.as_deref(),
    )
    .await
    .map_err(|e| {
        error!("insert diet_chat_history failed: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Cannot save chat history".to_string(),
            }),
        )
    })?;

    if let Err(e) =
        upsert_chat_room_meta(&state, auth_user.user_id, target_room_id, &original_message).await
    {
        // Do not fail the chat request if room metadata table has not been migrated yet.
        error!("upsert chat_rooms failed: {:?}", e);
    }

    let payload = NodeAgentPayload {
        message: request.message,
        thread_id: target_room_id.to_string(),
        chat_history_id: chat_history_id.to_string(),
        user_id: auth_user.user_id.to_string(),
        user_context: request.user_context,
        image: request.image,
    };

    let node_api_url = env::var(ENVKey::AGENT_API_URL).map_err(|e| {
        error!("cannot get env value {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Server error".to_string(),
            }),
        )
    })?;

    let res = client
        .post(node_api_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            error!("forward to Node.js Agent failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "AI request failed".into(),
                }),
            )
        })?;

    let stream = res.bytes_stream().map(|chunk| match chunk {
        Ok(bytes) => {
            let text = String::from_utf8_lossy(&bytes).to_string();
            let clean_json = text.replace("data: ", "").trim().to_string();
            if clean_json.is_empty() {
                Ok(Event::default().data(""))
            } else {
                Ok(Event::default().data(clean_json))
            }
        }
        Err(e) => {
            error!("stream read failed: {:?}", e);
            Ok(Event::default().data(r#"{"type":"error","content":"Stream read failed"}"#))
        }
    });

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive-text"),
    ))
}

async fn persist_user_chat_record(
    state: &Arc<AppState>,
    user_id: Uuid,
    room_id: Uuid,
    user_message: &str,
    image_path: Option<&str>,
) -> Result<Uuid, sqlx::Error> {
    let history_id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO diet_chat_history (id, room_id, user_id, user_message, image_path, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "#,
    )
    .bind(history_id)
    .bind(room_id.to_string())
    .bind(user_id)
    .bind(user_message)
    .bind(image_path)
    .execute(&state.db)
    .await?;

    Ok(history_id)
}

async fn upsert_chat_room_meta(
    state: &Arc<AppState>,
    user_id: Uuid,
    room_id: Uuid,
    message: &str,
) -> Result<(), sqlx::Error> {
    let title_seed = if message.chars().count() > 40 {
        format!("{}...", message.chars().take(40).collect::<String>())
    } else {
        message.to_string()
    };

    sqlx::query(
        r#"
        INSERT INTO chat_rooms (room_id, user_id, title, summary, created_at, updated_at, last_message_at)
        VALUES ($1, $2, $3, '[]'::jsonb, NOW(), NOW(), NOW())
        ON CONFLICT (room_id, user_id)
        DO UPDATE SET
            updated_at = NOW(),
            last_message_at = NOW()
        "#,
    )
    .bind(room_id.to_string())
    .bind(user_id)
    .bind(title_seed)
    .execute(&state.db)
    .await?;

    Ok(())
}

fn guess_ext_from_data_url(header: &str) -> &'static str {
    if header.contains("image/png") {
        "png"
    } else if header.contains("image/jpeg") || header.contains("image/jpg") {
        "jpg"
    } else if header.contains("image/webp") {
        "webp"
    } else if header.contains("image/gif") {
        "gif"
    } else {
        "png"
    }
}

async fn save_chat_image_if_needed(
    user_id: &Uuid,
    room_id: &Uuid,
    image: Option<String>,
) -> Result<Option<String>, String> {
    let Some(image) = image else {
        return Ok(None);
    };

    if Path::new(&image).exists() {
        return Ok(Some(image));
    }

    let (ext, base64_body) = if image.starts_with("data:") {
        let (header, body) = image
            .split_once(',')
            .ok_or_else(|| "invalid data url".to_string())?;
        (guess_ext_from_data_url(header), body)
    } else {
        ("png", image.as_str())
    };

    let decoded = general_purpose::STANDARD
        .decode(base64_body)
        .map_err(|_| "base64 decode failed".to_string())?;

    let upload_root = env::var(ENVKey::CHAT_IMAGE_UPLOAD_DIR)
        .unwrap_or_else(|_| "uploads/chat_images".to_string());
    let folder = PathBuf::from(upload_root)
        .join(user_id.to_string())
        .join(room_id.to_string());
    fs::create_dir_all(&folder)
        .await
        .map_err(|e| format!("create dir failed: {e}"))?;

    let filename = format!("{}.{}", Uuid::new_v4(), ext);
    let file_path = folder.join(filename);
    fs::write(&file_path, decoded)
        .await
        .map_err(|e| format!("write image failed: {e}"))?;

    let absolute_path = match fs::canonicalize(&file_path).await {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => file_path.to_string_lossy().to_string(),
    };

    Ok(Some(absolute_path))
}
