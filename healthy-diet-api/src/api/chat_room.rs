use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
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
}

pub async fn get_chat_rooms_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let rooms = sqlx::query!(
        r#"
        SELECT
            room_id,
            MAX(created_at) as last_updated
        FROM diet_chat_history
        WHERE user_id = $1
        GROUP BY room_id
        ORDER BY last_updated DESC
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

    let room_responses: Vec<RoomResponse> = rooms
        .into_iter()
        .map(|r| {
            let room_id_str = r.room_id;

            let short_id: String = room_id_str.chars().take(6).collect();

            RoomResponse {
                id: room_id_str,
                title: format!("諮詢室 #{}", short_id),
                last_updated: r.last_updated,
            }
        })
        .collect();

    Ok((StatusCode::OK, Json(json!({ "rooms": room_responses }))))
}

pub async fn get_room_history_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let records = sqlx::query!(
        r#"
        SELECT user_message, ai_analysis_report
        FROM diet_chat_history
        WHERE room_id = $1 AND user_id = $2
        ORDER BY created_at ASC
        "#,
        room_id,
        auth_user.user_id
    )
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
        if let Some(msg) = record.user_message {
            history.push(ChatMessage {
                role: "user".to_string(),
                content: msg,
            });
        }

        if let Some(ai_msg) = record.ai_analysis_report {
            history.push(ChatMessage {
                role: "ai".to_string(),
                content: ai_msg,
            });
        }
    }

    Ok((StatusCode::OK, Json(json!({ "history": history }))))
}
