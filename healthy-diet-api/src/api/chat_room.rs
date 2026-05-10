use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use sqlx::Row;
use std::sync::Arc;
use tokio::fs;
use tracing::error;

use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};

#[derive(Serialize)]
pub struct RoomResponse {
    pub id: String,
    pub title: String,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_base64: Option<String>,
}

pub async fn get_chat_rooms_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let rooms = sqlx::query!(
        r#"
        SELECT DISTINCT ON (room_id)
            room_id,
            user_message as title,
            created_at as last_updated
        FROM diet_chat_history
        WHERE user_id = $1
        ORDER BY room_id, created_at ASC
        "#,
        auth_user.user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("查詢聊天室列表失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "資料庫查詢失敗".into(),
            }),
        )
    })?;

    let mut room_responses: Vec<RoomResponse> = rooms
        .into_iter()
        .map(|r| {
            let display_title = r
                .title
                .map(|t| {
                    if t.chars().count() > 20 {
                        format!("{}...", t.chars().take(20).collect::<String>())
                    } else {
                        t
                    }
                })
                .unwrap_or_else(|| "新對話".to_string());

            RoomResponse {
                id: r.room_id,
                title: display_title,
                last_updated: r.last_updated,
            }
        })
        .collect();

    room_responses.sort_by(|a, b| b.last_updated.cmp(&a.last_updated));

    Ok((StatusCode::OK, Json(json!({ "rooms": room_responses }))))
}

pub async fn get_room_history_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let records = sqlx::query(
        r#"
        SELECT user_message, ai_analysis_report, image_path
        FROM diet_chat_history
        WHERE room_id = $1 AND user_id = $2
        ORDER BY created_at ASC
        "#,
    )
    .bind(&room_id)
    .bind(auth_user.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("查詢歷史紀錄失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "資料庫查詢失敗".into(),
            }),
        )
    })?;

    let mut history: Vec<ChatMessage> = Vec::new();

    for record in records {
        let user_message: Option<String> = record.try_get("user_message").ok().flatten();
        let ai_analysis_report: Option<String> =
            record.try_get("ai_analysis_report").ok().flatten();
        let image_path: Option<String> = record.try_get("image_path").ok().flatten();
        let image_base64 = load_image_base64(image_path.as_deref()).await;

        if let Some(msg) = user_message {
            history.push(ChatMessage {
                role: "user".to_string(),
                content: msg,
                image_path,
                image_base64,
            });
        }

        if let Some(ai_msg) = ai_analysis_report {
            history.push(ChatMessage {
                role: "ai".to_string(),
                content: ai_msg,
                image_path: None,
                image_base64: None,
            });
        }
    }

    Ok((StatusCode::OK, Json(json!({ "history": history }))))
}

fn guess_mime_from_path(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else {
        "application/octet-stream"
    }
}

async fn load_image_base64(image_path: Option<&str>) -> Option<String> {
    let path = image_path?;
    let bytes = fs::read(path).await.ok()?;
    let base64_data = general_purpose::STANDARD.encode(bytes);
    let mime = guess_mime_from_path(path);
    Some(format!("data:{};base64,{}", mime, base64_data))
}
