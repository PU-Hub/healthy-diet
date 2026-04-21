use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::Multipart;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs, process::Command, sync::Arc};
use tracing::error;
use uuid::Uuid;

use crate::{api::model::ErrorResponse, model::AppState, utils::jwt::AuthUser};

// --- YOLO 腳本輸出的反序列化結構 ---
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

// --- 萃取出的乾淨食物特徵 (準備存入 DB 與回傳前端) ---
#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractedFoodItem {
    pub class: String,
    pub confidence: f64,
    pub area_ratio: f64, // 佔總食物體積的比例 (方案 A)
}

// --- 回傳給前端的最終 JSON 結構 ---
#[derive(Serialize)]
pub struct VisionResponse {
    pub message: String,
    pub draft_id: String, // 草稿單號 (供前端呼叫 Agent 使用)
    pub image_base64: Option<String>,
    pub items: Vec<ExtractedFoodItem>,
}

pub async fn yolo_handler(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<VisionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 1. 驗證使用者是否存在 (保留你原本的安全機制)
    let user_profile = sqlx::query!("SELECT id FROM users WHERE id = $1", auth_user.user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("DB 錯誤: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "資料庫連線錯誤".into(),
                }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "找不到使用者".into(),
            }),
        ))?;

    // 2. 接收並儲存上傳的圖片
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

    // 產生任務單號 (此 UUID 將貫穿整個草稿與正式紀錄)
    let draft_id = Uuid::new_v4();
    let input_path = format!("/app/uploads/{}.jpg", draft_id);

    fs::create_dir_all("/app/uploads").ok();
    fs::write(&input_path, data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "伺服器儲存檔案失敗".into(),
            }),
        )
    })?;

    // 3. 呼叫 YOLO 辨識腳本
    let yolo_script = env::var("YOLO_SCRIPT_PATH")
        .unwrap_or_else(|_| "../healthy-diet-yolo/predict.py".to_string());

    let output = Command::new("python3")
        .arg(&yolo_script)
        .arg("--input")
        .arg(&input_path)
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
        let stderror = String::from_utf8_lossy(&output.stderr);
        error!("!辨識過程出錯: {:?}", stderror);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "YOLO 辨識過程出現錯誤".into(),
            }),
        ));
    }

    // 4. 解析 YOLO JSON 輸出
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let json_start = stdout_str.find('{').unwrap_or(0);
    let json_end = stdout_str
        .rfind('}')
        .unwrap_or(stdout_str.len().saturating_sub(1))
        + 1;
    let clean_json = if json_start < json_end {
        &stdout_str[json_start..json_end]
    } else {
        "{}"
    };

    let yolo_result: YoloScriptOutput = serde_json::from_str(clean_json).map_err(|e| {
        error!("解析 YOLO 失敗: {}, 原始輸出: {}", e, stdout_str);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "解析辨識結果失敗".into(),
            }),
        )
    })?;

    // ==========================================
    // 核心演算法：方案 A (體積貢獻指數法)
    // ==========================================

    // 步驟 1: 計算所有食物框的面積總和
    let mut total_food_area = 0.0;
    for det in &yolo_result.detections {
        let width = det.bbox[2] - det.bbox[0];
        let height = det.bbox[3] - det.bbox[1];
        total_food_area += width * height;
    }

    let mut extracted_items = Vec::new();

    // 步驟 2: 計算每個物件的相對面積佔比
    if total_food_area > 0.0 {
        for det in yolo_result.detections {
            let width = det.bbox[2] - det.bbox[0];
            let height = det.bbox[3] - det.bbox[1];
            let bbox_area = width * height;

            // 計算比例，並四捨五入到小數點後三位
            let area_ratio = bbox_area / total_food_area;
            let rounded_ratio = (area_ratio * 1000.0).round() / 1000.0;

            extracted_items.push(ExtractedFoodItem {
                class: det.class_name,
                confidence: (det.confidence * 100.0).round() / 100.0,
                area_ratio: rounded_ratio,
            });
        }
    } else {
        // 如果完全沒有辨識到任何食物，提早結束並回傳提示
        let image_base64 = fs::read(&yolo_result.image_path)
            .ok()
            .map(|b| general_purpose::STANDARD.encode(b));
        return Ok(Json(VisionResponse {
            message: "未偵測到明顯的食物，請重新拍攝。".into(),
            draft_id: draft_id.to_string(),
            image_base64,
            items: vec![],
        }));
    }

    // ==========================================
    // 資料庫存檔：寫入 diet_drafts 草稿表
    // ==========================================

    // 將整理好的陣列轉為 JSONB 格式
    let items_jsonb = serde_json::to_value(&extracted_items).unwrap_or(json!([]));

    let insert_result = sqlx::query!(
        r#"INSERT INTO diet_drafts (id, user_id, image_path, detected_items)
           VALUES ($1, $2, $3, $4)"#,
        draft_id,
        auth_user.user_id,
        &yolo_result.image_path,
        items_jsonb
    )
    .execute(&state.db)
    .await;

    if let Err(e) = insert_result {
        error!("存入草稿失敗: {:?}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "資料庫草稿儲存失敗".into(),
            }),
        ));
    }

    let image_base64 = fs::read(&yolo_result.image_path)
        .ok()
        .map(|b| general_purpose::STANDARD.encode(b));

    Ok(Json(VisionResponse {
        message: "辨識完成！請確認食材資訊。".into(),
        draft_id: draft_id.to_string(),
        image_base64,
        items: extracted_items,
    }))
}
