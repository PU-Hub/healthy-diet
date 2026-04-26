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

#[axum::debug_handler]
pub async fn diet_image_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    auth_user: Option<AuthUser>,
    Json(payload): Json<ImageRequest>,
) -> Result<Json<ImageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let n8n_secret =
        env::var("N8N_SECRET_KEY").unwrap_or_else(|_| "N8N_SUPER_SECRET_STATIC_KEY".to_string());

    let n8n_token = format!("Bearer {}", n8n_secret);


    let is_n8n = headers
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .map(|auth_str| auth_str == n8n_token)
        .unwrap_or(false);

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
