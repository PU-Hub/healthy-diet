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
    // 1. 發送請求給 Node.js 討標題
    let client = Client::new();
    let agent_api_url = env::var(ENVKey::AGENT_API_URL).expect("[generate title]Miss env");
    let normalized_url = agent_api_url.trim_end_matches('/');
    let node_api_url = if normalized_url.ends_with("/api/chat") {
        format!(
            "{}/generate_title",
            normalized_url.trim_end_matches("/api/chat")
        )
    } else if normalized_url.ends_with("/chat") {
        format!("{}/generate_title", normalized_url.trim_end_matches("/chat"))
    } else {
        format!("{normalized_url}/generate_title")
    };

    let res = client
        .post(node_api_url)
        .json(&json!({ "message": payload.message }))
        .send()
        .await
        .map_err(|e| {
            error!("請求 Node.js 生成標題失敗: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "標題生成服務異常".into(),
                }),
            )
        })?;

    // 2. 解析 Node.js 回傳的標題
    let node_data: NodeTitleResponse = res.json().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "解析標題失敗".into(),
            }),
        )
    })?;

    let new_title = node_data.title;

    // 3. 將新標題更新到資料庫中該 room_id 的所有紀錄
    sqlx::query!(
        r#"
        UPDATE diet_chat_history
        SET title = $1
        WHERE room_id = $2 AND user_id = $3
        "#,
        new_title,
        room_id,
        auth_user.user_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("更新標題至資料庫失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "資料庫更新失敗".into(),
            }),
        )
    })?;

    // 4. 回傳新標題給前端更新畫面
    Ok((StatusCode::OK, Json(json!({ "title": new_title }))))
}
