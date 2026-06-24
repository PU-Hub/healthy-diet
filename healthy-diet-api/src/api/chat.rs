use axum::{
    extract::Json,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::sse::{Event, Sse},
};
use futures::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::{convert::Infallible, time::Duration};
use tracing::error;
use uuid::Uuid;

use crate::{
    api::model::ErrorResponse,
    model::ENVKey,
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
    pub is_new_conversation: bool,
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

pub async fn chat_check_handler() -> (StatusCode, Json<ProxyChatCheckResponse>) {
    let node_api_url = match env::var(ENVKey::AGENT_API_URL) {
        Ok(url) => url,
        Err(e) => {
            error!("cannot get env value {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ProxyChatCheckResponse {
                    ping_url: "a".to_string(),
                    ping_ok: false,
                    proxy_chat_available: false,
                    ping_status_code: None,
                    ping_response: Some(format!("lost environment variable: {}", e)),
                }),
            );
        }
    };

    let ping_url = format!("{}/ping", node_api_url);
    let client = Client::new();

    match client
        .get(&ping_url)
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

pub async fn chat_handler(
    headers: HeaderMap,
    auth_user: AuthUser,
    Json(request): Json<AgentChatRequest>,
) -> Result<
    Sse<impl futures::stream::Stream<Item = Result<Event, Infallible>>>,
    (StatusCode, Json<ErrorResponse>),
> {
    let client = Client::new();

    let is_new_conversation = request.room_id.is_none();
    let target_room_id = request.room_id.unwrap_or_else(Uuid::new_v4);

    let payload = NodeAgentPayload {
        message: request.message,
        thread_id: target_room_id.to_string(),
        is_new_conversation,
        user_id: auth_user.user_id.to_string(),
        user_context: request.user_context,
        image: request.image,
    };

    let node_api_base_url = env::var(ENVKey::AGENT_API_URL).map_err(|e| {
        error!("cannot get env value {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Server error".to_string(),
            }),
        )
    })?;

    let node_api_url = format!("{}/api/chat", node_api_base_url);

    let mut agent_request = client.post(node_api_url).json(&payload);
    if let Some(auth_value) = headers.get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_value.to_str() {
            agent_request = agent_request.header(AUTHORIZATION.as_str(), auth_str);
        }
    }

    let res = agent_request.send().await.map_err(|e| {
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

#[cfg(test)]
mod tests {
    use super::NodeAgentPayload;
    use uuid::Uuid;

    #[test]
    fn node_agent_payload_only_contains_proxy_fields() {
        let payload = NodeAgentPayload {
            message: "hello".to_string(),
            thread_id: Uuid::nil().to_string(),
            is_new_conversation: true,
            user_id: Uuid::nil().to_string(),
            user_context: None,
            image: Some("data:image/png;base64,abc".to_string()),
        };

        let json = serde_json::to_value(payload).expect("payload should serialize");

        assert_eq!(json.get("message").and_then(|v| v.as_str()), Some("hello"));
        assert_eq!(
            json.get("thread_id").and_then(|v| v.as_str()),
            Some(Uuid::nil().to_string().as_str())
        );
        assert_eq!(
            json.get("is_new_conversation").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert_eq!(
            json.get("image").and_then(|v| v.as_str()),
            Some("data:image/png;base64,abc")
        );
        assert!(
            json.get("chat_history_id").is_none(),
            "proxy payload should not pre-allocate chat history ids in Rust"
        );
    }
}
