use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::Multipart;
use std::{env, fs, process::Command, sync::Arc};
use tracing::{error, info};
use uuid::Uuid;

use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};

#[derive(serde::Serialize)]
pub struct YoloResponse {
    pub message: String,
    pub detected_objects: String,
}

pub async fn yolo_handler(
    _auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<YoloResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut image_data = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("讀取欄位失敗: {}", e),
            }),
        )
    })? {
        if field.name() == Some("image") {
            image_data = Some(field.bytes().await.map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "無法讀取圖片數據".into(),
                    }),
                )
            })?);
            break;
        }
    }

    let data = image_data.ok_or((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "請上傳圖片".into(),
        }),
    ))?;

    let file_id = Uuid::new_v4().to_string();
    let input_path = format!("./uploads/{}.jpg", file_id);

    fs::create_dir_all("./uploads").ok();
    fs::write(&input_path, data).map_err(|e| {
        error!("檔案寫入失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "伺服器儲存檔案失敗".into(),
            }),
        )
    })?;

    let yolo_script = env::var("YOLO_SCRIPT_PATH")
        .unwrap_or_else(|_| "../health-diet-yolo/predict.py".to_string());

    info!(
        "正在執行 YOLO 辨識，腳本路徑: {}, 圖片: {}",
        yolo_script, input_path
    );

    let output = Command::new("python3")
        .arg(yolo_script)
        .arg("predict")
        .arg("model=yolo11n.pt")
        .arg(format!("source={}", input_path))
        .arg("save=true")
        .output()
        .map_err(|e| {
            error!("YOLO CLI 執行錯誤: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "辨識引擎啟動失敗".into(),
                }),
            )
        })?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        error!("YOLO 執行失敗: {}", err_msg);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "YOLO 辨識過程出錯".into(),
            }),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(Json(YoloResponse {
        message: "辨識完成".into(),
        detected_objects: stdout,
    }))
}
