use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use std::{env, sync::Arc};
use tracing::error; // 🌟 記得引入 json 巨集

use crate::{
    api::model::{ConsultRequest, ConsultResponse, ErrorResponse},
    model::{AppState, ENVKey, OutSideURL},
    utils::{ai_prompt::build_xml_system_prompt, jwt::AuthUser},
};

pub async fn consult_handler(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ConsultRequest>,
) -> Result<Json<ConsultResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 1. 取得使用者資料 (包含新的 age, gender, taboo, disease)
    let user_profile = sqlx::query!(
        "SELECT nickname, height, weight, age, gender, taboo, disease FROM users WHERE id = $1",
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

    // 2. 取得 API Key
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

    // 3. 將資料組裝為 XML 格式
    let taboo_str = user_profile.taboo.unwrap_or_default().join("、");
    let disease_str = user_profile.disease.unwrap_or_default().join("、");

    let profile_text = format!(
        "<basic_info>\n\
        \x20\x20<nickname>{}</nickname>\n\
        \x20\x20<gender>{}</gender>\n\
        \x20\x20<age>{}</age>\n\
        \x20\x20<height_cm>{}</height_cm>\n\
        \x20\x20<weight_kg>{}</weight_kg>\n\
        </basic_info>\n\
        <health_status>\n\
        \x20\x20<diseases>{}</diseases>\n\
        \x20\x20<taboos>{}</taboos>\n\
        </health_status>",
        user_profile
            .nickname
            .unwrap_or_else(|| "使用者".to_string()),
        user_profile.gender.unwrap_or_else(|| "未提供".to_string()),
        user_profile
            .age
            .map(|v| v.to_string())
            .unwrap_or_else(|| "未提供".to_string()),
        user_profile
            .height
            .map(|v| v.to_string())
            .unwrap_or_else(|| "未提供".to_string()),
        user_profile
            .weight
            .map(|v| v.to_string())
            .unwrap_or_else(|| "未提供".to_string()),
        if disease_str.is_empty() {
            "無".to_string()
        } else {
            disease_str
        },
        if taboo_str.is_empty() {
            "無".to_string()
        } else {
            taboo_str
        }
    );

    // 4. 從 AppState 讀取 AI Prompt 設定，產生 System Prompt
    let system_prompt = build_xml_system_prompt(&state.ai_prompt_config, &profile_text);

    // 5. 取得過去 5 筆歷史對話 (讓 AI 有記憶)
    let history_records = sqlx::query!(
        r#"SELECT question, ai_response FROM ai_consultations
           WHERE user_id = $1 ORDER BY id DESC LIMIT 5"#,
        auth_user.user_id
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut contents: Vec<serde_json::Value> = Vec::new();

    for record in history_records.into_iter().rev() {
        contents.push(json!({ "role": "user", "parts": [{ "text": record.question }] }));
        contents.push(json!({ "role": "model", "parts": [{ "text": record.ai_response }] }));
    }

    contents.push(json!({ "role": "user", "parts": [{ "text": format!("<user_input>\n{}\n</user_input>", payload.question) }] }));

    let request_payload = json!({
        "system_instruction": { "parts": { "text": system_prompt } },
        "contents": contents
    });

    // 7. 發送至 Gemini API
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&request_payload)
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

    let res_body: serde_json::Value = response.json().await.map_err(|e| {
        error!("Fail to parse Json: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "AI Response Error".into(),
            }),
        )
    })?;

    let ai_reply = res_body["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("抱歉我現在無法回答問題")
        .to_string();

    // 9. 儲存本次對話至資料庫
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

/*

⡴⠑⡄⠀⠀⠀⠀⠀⠀⠀ ⣀⣀⣤⣤⣤⣀⡀
⠸⡇⠀⠿⡀⠀⠀⠀⣀⡴⢿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀
⠀⠀⠀⠀⠑⢄⣠⠾⠁⣀⣄⡈⠙⣿⣿⣿⣿⣿⣿⣿⣿⣆
⠀⠀⠀⠀⢀⡀⠁⠀⠀⠈⠙⠛⠂⠈⣿⣿⣿⣿⣿⠿⡿⢿⣆
⠀⠀⠀⢀⡾⣁⣀⠀⠴⠂⠙⣗⡀⠀⢻⣿⣿⠭⢤⣴⣦⣤⣹⠀⠀⠀⢀⢴⣶⣆
⠀⠀⢀⣾⣿⣿⣿⣷⣮⣽⣾⣿⣥⣴⣿⣿⡿⢂⠔⢚⡿⢿⣿⣦⣴⣾⠸⣼⡿
⠀⢀⡞⠁⠙⠻⠿⠟⠉⠀⠛⢹⣿⣿⣿⣿⣿⣌⢤⣼⣿⣾⣿⡟⠉
⠀⣾⣷⣶⠇⠀⠀⣤⣄⣀⡀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇
⠀⠉⠈⠉⠀⠀⢦⡈⢻⣿⣿⣿⣶⣶⣶⣶⣤⣽⡹⣿⣿⣿⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠉⠲⣽⡻⢿⣿⣿⣿⣿⣿⣿⣷⣜⣿⣿⣿⡇
⠀⠀ ⠀⠀⠀⠀⠀⢸⣿⣿⣷⣶⣮⣭⣽⣿⣿⣿⣿⣿⣿⣿⠇
⠀⠀⠀⠀⠀⠀⣀⣀⣈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇
⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃

*/
