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
use tokio::{fs, time::sleep};
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

    let payload = NodeAgentPayload {
        message: request.message,
        thread_id: target_room_id.to_string(),
        user_id: auth_user.user_id.to_string(),
        user_context: request.user_context,
        image: request.image,
    };

    let node_api_url = format!(
        "{}",
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

    let res = client
        .post(node_api_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            error!("無法連線到 Node.js Agent: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "AI 伺服器連線失敗，請稍後再試".into(),
                }),
            )
        })?;

    if let Some(image_path) = saved_image_path {
        let db = state.db.clone();
        let user_id = auth_user.user_id;
        let room_id = target_room_id.to_string();
        tokio::spawn(async move {
            let mut retries = 0u8;
            while retries < 20 {
                let update_result = sqlx::query(
                    r#"
                    UPDATE diet_chat_history
                    SET image_path = $1
                    WHERE ctid IN (
                        SELECT ctid
                        FROM diet_chat_history
                        WHERE room_id = $2
                          AND user_id = $3
                          AND user_message = $4
                        ORDER BY created_at DESC
                        LIMIT 1
                    )
                    "#,
                )
                .bind(&image_path)
                .bind(&room_id)
                .bind(user_id)
                .bind(&original_message)
                .execute(&db)
                .await;

                match update_result {
                    Ok(result) if result.rows_affected() > 0 => return,
                    Ok(_) => {
                        retries += 1;
                        sleep(Duration::from_millis(500)).await;
                    }
                    Err(e) => {
                        error!("update diet_chat_history image_path failed: {:?}", e);
                        return;
                    }
                }
            }
        });
    }

    let stream = res.bytes_stream().map(|chunk| match chunk {
        Ok(bytes) => {
            let text = String::from_utf8_lossy(&bytes).to_string();

            let clean_json = text.replace("data: ", "").trim().to_string();

            if !clean_json.is_empty() {
                Ok(Event::default().data(clean_json))
            } else {
                Ok(Event::default().data(""))
            }
        }
        Err(e) => {
            error!("讀取 Stream 發生錯誤: {:?}", e);
            Ok(Event::default().data(r#"{"type":"error","content":"Stream 中斷"}"#))
        }
    });

    // 回傳 Sse 結構，並設定 15 秒的心跳包 (Keep-Alive) 避免連線中斷
    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive-text"),
    ))
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
