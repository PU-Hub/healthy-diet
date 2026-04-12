use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};
use axum::{Json, extract::State, http::StatusCode};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ImageResponse {
    pub message: String,
    pub image_base64: Option<String>,
}
#[derive(Deserialize)]
pub struct ImageRequest {
    pub record_id: Uuid,
}

pub async fn diet_image_handler(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ImageRequest>,
) -> Result<Json<ImageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let record = sqlx::query!(
        "SELECT result_image_path FROM diet_records WHERE id = $1 AND user_id = $2",
        payload.record_id,
        auth_user.user_id
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
    .ok_or((
        StatusCode::FORBIDDEN,
        Json(ErrorResponse {
            error: "無權限或紀錄不存在".into(),
        }),
    ))?;

    let path_str = record.result_image_path.ok_or((
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
