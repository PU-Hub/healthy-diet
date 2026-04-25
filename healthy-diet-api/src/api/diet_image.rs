use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header},
};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;
// 記得引入 env 模組
use std::env;

#[derive(Serialize)]
pub struct ImageResponse {
    pub message: String,
    pub image_base64: Option<String>,
}

#[derive(Deserialize)]
pub struct ImageRequest {
    pub record_id: Uuid,
}

// 加入這個巨集！它會幫你把編譯錯誤翻譯成人類看得懂的語言
#[axum::debug_handler]
pub async fn diet_image_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    auth_user: Option<AuthUser>, // 確保自定義提取器在 Json 前面
    Json(payload): Json<ImageRequest>,
) -> Result<Json<ImageResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 從 .env 讀取密碼，如果沒設定就使用預設值
    let n8n_secret =
        env::var("N8N_SECRET_KEY").unwrap_or_else(|_| "N8N_SUPER_SECRET_STATIC_KEY".to_string());

    // 組合成完整的 Bearer Token 格式
    let n8n_token = format!("Bearer {}", n8n_secret);

    // 驗證是否為 n8n 的請求
    let is_n8n = headers
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .map(|auth_str| auth_str == n8n_token)
        .unwrap_or(false);

    // 權限判斷：既不是 n8n，也沒有合法的使用者 Token
    if !is_n8n && auth_user.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "未授權的請求".into(),
            }),
        ));
    }

    let result_image_path = if is_n8n {
        sqlx::query!(
            "SELECT result_image_path FROM diet_records WHERE id = $1",
            payload.record_id
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("DB 錯誤: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "資料庫查詢失敗".into(),
                }),
            )
        })?
        .map(|r| r.result_image_path)
    } else {
        let user = auth_user.unwrap();
        sqlx::query!(
            "SELECT result_image_path FROM diet_records WHERE id = $1 AND user_id = $2",
            payload.record_id,
            user.user_id
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("DB 錯誤: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "資料庫查詢失敗".into(),
                }),
            )
        })?
        .map(|r| r.result_image_path)
    }
    .ok_or((
        StatusCode::FORBIDDEN,
        Json(ErrorResponse {
            error: "無權限或紀錄不存在".into(),
        }),
    ))?;

    let path_str = result_image_path.ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "此紀錄未包含圖片路徑".into(),
        }),
    ))?;

    let path = std::path::Path::new(&path_str);
    if !path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "伺服器檔案已遺失".into(),
            }),
        ));
    }

    let bytes = tokio::fs::read(path).await.map_err(|e| {
        error!("讀取失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "讀取圖片失敗".into(),
            }),
        )
    })?;

    let base64_data = general_purpose::STANDARD.encode(bytes);

    Ok(Json(ImageResponse {
        message: "讀取成功".into(),
        image_base64: Some(base64_data),
    }))
}
