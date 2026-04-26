use axum::{extract::State, http::StatusCode, response::IntoResponse, Json}; // 將 Json 加在這裡
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

use crate::{
    api::model::ErrorResponse,
    model::AppState,
    utils::jwt::AuthUser,
};

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

    let chat_record = sqlx::query!(
      r#"
              INSERT INTO diet_chat_history (room_id, title, user_id, diet_record_id, user_message, sender_role)
              VALUES ($1, $2, $3, $4, $5, $6)
              RETURNING id
              "#,
              target_room_id,
              "新對話",
              auth_user.user_id,
              request.diet_record_id,
              request.user_message,
              "user"
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("建立聊天紀錄失敗: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "資料庫建立紀錄失敗".into() }))
    })?;

    Ok((StatusCode::CREATED, Json(json!({
        "chat_id": chat_record.id,
        "room_id": target_room_id
    }))))
}
