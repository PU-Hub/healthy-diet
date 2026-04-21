use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::Multipart;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs, process::Command, sync::Arc};
use tracing::error;
use uuid::Uuid;

use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};

#[derive(Deserialize, Debug)]
pub struct YoloScriptOutput {
    pub status: String,
    pub image_path: String,
    pub json_path: String,
    pub detections: Vec<Detection>,
}

#[derive(Deserialize, Debug)]
pub struct Detection {
    #[serde(rename = "class")]
    pub class_name: String,
    pub confidence: f64,
    pub bbox: [f64; 4], // [x_min, y_min, x_max, y_max]
}

// 準備存入資料庫與回傳前端的乾淨資料
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedFoodItem {
    pub class: String,
    pub confidence: f64,
    pub area_ratio: f64, // 方案 A：佔總食物體積（面積加總）之比例
}

// 回傳給前端的 JSON 結構
#[derive(Serialize)]
pub struct VisionResponse {
    pub message: String,
    pub draft_id: String,             // 重要：後續 Agent 修正用的 Key
    pub image_base64: Option<String>, // 給使用者看是否有框選錯誤
    pub items: Vec<ExtractedFoodItem>,
}

pub async fn yolo_handler(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<VisionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 1. 讀取圖片數據
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

    let draft_id = Uuid::new_v4();
    let input_path = format!("/app/uploads/{}.jpg", draft_id);

    fs::create_dir_all("/app/uploads").ok();
    fs::write(&input_path, data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "伺服器儲存圖片失敗".into(),
            }),
        )
    })?;

    // 2. 執行 YOLO 辨識
    let yolo_script = env::var("YOLO_SCRIPT_PATH")
        .unwrap_or_else(|_| "../healthy-diet-yolo/predict.py".to_string());

    let output = Command::new("python3")
        .arg(&yolo_script)
        .arg("--input")
        .arg(&input_path)
        .output()
        .map_err(|e| {
            error!("YOLO 執行錯誤: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "辨識引擎啟動失敗".into(),
                }),
            )
        })?;

    if !output.status.success() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "YOLO 辨識出錯".into(),
            }),
        ));
    }

    // 3. 解析結果並計算佔比 (方案 A)
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let json_start = stdout_str.find('{').unwrap_or(0);
    let json_end = stdout_str.rfind('}').unwrap_or(stdout_str.len()) + 1;
    let yolo_result: YoloScriptOutput = serde_json::from_str(&stdout_str[json_start..json_end])
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "解析結果失敗".into(),
                }),
            )
        })?;

    let mut total_food_area = 0.0;
    for det in &yolo_result.detections {
        total_food_area += (det.bbox[2] - det.bbox[0]) * (det.bbox[3] - det.bbox[1]);
    }

    let mut extracted_items = Vec::new();
    if total_food_area > 0.0 {
        for det in yolo_result.detections {
            let bbox_area = (det.bbox[2] - det.bbox[0]) * (det.bbox[3] - det.bbox[1]);
            extracted_items.push(ExtractedFoodItem {
                class: det.class_name,
                confidence: (det.confidence * 100.0).round() / 100.0,
                area_ratio: ((bbox_area / total_food_area) * 1000.0).round() / 1000.0,
            });
        }
    }

    let items_jsonb = serde_json::to_value(&extracted_items).unwrap_or(json!([]));
    sqlx::query!(
        "INSERT INTO diet_drafts (id, user_id, image_path, detected_items) VALUES ($1, $2, $3, $4)",
        draft_id,
        auth_user.user_id,
        &yolo_result.image_path, // 儲存畫好框的圖片路徑
        items_jsonb
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("資料庫草稿存入失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "草稿存入失敗".into(),
            }),
        )
    })?;

    let image_base64 = fs::read(&yolo_result.image_path)
        .ok()
        .map(|b| general_purpose::STANDARD.encode(b));

    Ok(Json(VisionResponse {
        message: "辨識完成，請確認食材是否有誤。".into(),
        draft_id: draft_id.to_string(),
        image_base64,
        items: extracted_items,
    }))
}
