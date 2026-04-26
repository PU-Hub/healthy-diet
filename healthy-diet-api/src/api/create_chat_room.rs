use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub diet_record_id: Uuid,
    pub user_message: String,
    pub room_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatHistory {
    pub id: Option<Uuid>,
    pub room_id: Uuid,
    pub title: String,
    pub user_id: Uuid,
    pub user_message: String,
    pub sender_role: String,
}

pub async fn create_chat_room_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(request): Json<CreateChatRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let target_room_id = request.room_id.unwrap_or_else(Uuid::new_v4);

    let existing_room = sqlx::query!(
        r#"
        SELECT title, user_id, diet_record_id
        FROM diet_chat_history
        WHERE room_id = $1
        ORDER BY created_at ASC
        LIMIT 1
        "#,
        target_room_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("查詢房間失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "資料庫查詢失敗".into(),
            }),
        )
    })?;

    let (final_title, final_diet_id) = match existing_room {
        Some(room) => {
            if room.user_id != auth_user.user_id {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ErrorResponse {
                        error: "無權存取此聊天室".into(),
                    }),
                ));
            }
            (room.title, room.diet_record_id)
        }
        None => ("新對話".to_string(), Some(request.diet_record_id)),
    };

    let chat_record = sqlx::query!(
        r#"
        INSERT INTO diet_chat_history (room_id, title, user_id, diet_record_id, user_message)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        target_room_id,
        final_title,
        auth_user.user_id,
        final_diet_id,
        request.user_message
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("建立聊天紀錄失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "資料庫建立紀錄失敗".into(),
            }),
        )
    })?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "chat_id": chat_record.id,
            "room_id": target_room_id,
            "title": final_title
        })),
    ))
}
