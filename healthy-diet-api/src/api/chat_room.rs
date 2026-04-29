use axum::{
    extract::Json,
    http::StatusCode,
    response::sse::{Event, Sse},
};
use futures::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, time::Duration};
use tracing::error;
use uuid::Uuid;

use crate::{api::model::ErrorResponse, utils::jwt::AuthUser};

#[derive(Deserialize)]
pub struct AgentChatRequest {
    pub message: String,
    pub room_id: Option<Uuid>,
}

#[derive(Serialize)]
struct NodeAgentPayload {
    pub message: String,
    pub thread_id: String,
    pub user_id: String,
}

pub async fn proxy_agent_chat_handler(
    auth_user: AuthUser,
    Json(request): Json<AgentChatRequest>,
) -> Result<
    Sse<impl futures::stream::Stream<Item = Result<Event, Infallible>>>,
    (StatusCode, Json<ErrorResponse>),
> {
    let client = Client::new();

    let target_room_id = request.room_id.unwrap_or_else(Uuid::new_v4);

    let payload = NodeAgentPayload {
        message: request.message,
        thread_id: target_room_id.to_string(),
        user_id: auth_user.user_id.to_string(),
    };

    let node_api_url = "http://127.0.0.1:8001/api/chat";

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
