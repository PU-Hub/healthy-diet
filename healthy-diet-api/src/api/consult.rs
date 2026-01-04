use axum::{Json, extract::State, http::StatusCode};
use std::{env, sync::Arc};
use tracing::error;

use crate::{
    api::model::{ConsultRequest, ConsultResponse, ErrorResponse},
    model::{AppState, ENVKey, OutSideURL},
    utils::{
        gemini::{GeminiContent, GeminiPart, GeminiRequest, GeminiResponse},
        jwt::AuthUser,
    },
};

pub async fn consult_handler(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ConsultRequest>,
) -> Result<Json<ConsultResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user_profile = sqlx::query!(
        "SELECT nickname, height, weight, dietary_restrictions FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to fetch user profile for AI: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Database Error".into(),
            }),
        )
    })?
    .ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "User not found".into(),
        }),
    ))?;

    let api_key = env::var(ENVKey::GEMINI_API_KEY).map_err(|e| {
        error!("cannot get env value {:?}", e);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Server error".to_string(),
            }),
        )
    })?;

    let url = format!("{}{}", OutSideURL::GEMINI_API_URL, api_key);

    let profile_text = format!(
        "使用者暱稱：{}\n身高：{} cm\n體重：{} kg\n飲食禁忌/備註：{}",
        user_profile.nickname.unwrap_or("使用者".to_string()),
        user_profile
            .height
            .map(|v| v.to_string())
            .unwrap_or("未提供".to_string()),
        user_profile
            .weight
            .map(|v| v.to_string())
            .unwrap_or("未提供".to_string()),
        user_profile
            .dietary_restrictions
            .unwrap_or("無".to_string())
    );

    let system_instruction = format!(
        "你是一位專業的健康飲食顧問與營養師。請根據以下的【使用者檔案】與【使用者問題】，提供個人化、安全且專業的建議。\n\n【使用者檔案\n{}\n請注意：若使用者有飲食禁忌，絕對不能推薦相關食物。",
        profile_text
    );

    let full_prompt = format!("{}\n\n{}", system_instruction, payload.question);

    let request_body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart { text: full_prompt }],
        }],
    };

    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            error!("gemini api error: {:?}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "AI Sevice Unavailable".into(),
                }),
            )
        })?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();

        error!("Gemini API Error: {:?}", error_text);

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "AI Service Error".into(),
            }),
        ));
    }

    let gemini_data: GeminiResponse = response.json().await.map_err(|e| {
        error!("Fail to parse Json: {:?}", e);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "AI Response Error".into(),
            }),
        )
    })?;

    let ai_reply = gemini_data
        .candidates
        .and_then(|c| c.into_iter().next())
        .and_then(|c| c.content.parts.into_iter().next())
        .map(|p| p.text)
        .unwrap_or_else(|| "抱歉我現在無法回答問題".to_string());

    sqlx::query!(
        "INSERT INTO ai_consultations (user_id, question, ai_response) VALUES ($1, $2, $3)",
        auth_user.user_id,
        payload.question,
        ai_reply
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to save consultation: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Database Error".into(),
            }),
        )
    })?;

    Ok(Json(ConsultResponse { reply: ai_reply }))
}
