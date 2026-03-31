use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::Multipart;
use serde::{Deserialize, Serialize};
use std::{env, fs, process::Command, sync::Arc};
use tracing::{error, info};
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

#[derive(Serialize)]
pub struct CalorieResponse {
    pub message: String,
    pub total_calories: f64,
    pub detected_items: Vec<FoodItem>,
}

#[derive(Serialize)]
pub struct FoodItem {
    pub class: String,
    pub confidence: f64,
    pub estimated_weight_g: f64,
    pub calories: f64,
}

pub async fn yolo_handler(
    _auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<CalorieResponse>, (StatusCode, Json<ErrorResponse>)> {
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
    let input_path = format!("/app/uploads/{}.jpg", file_id);

    fs::create_dir_all("/app/uploads").ok();
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
        .unwrap_or_else(|_| "../healthy-diet-yolo/predict.py".to_string());

    info!(
        "正在執行 YOLO 辨識，腳本路徑: {}, 圖片: {}",
        yolo_script, input_path
    );

    let output = Command::new("python3")
        .arg(yolo_script)
        .arg("--input")
        .arg(format!("{}", input_path))
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

    let yolo_result: YoloScriptOutput = match serde_json::from_str(&stdout) {
        Ok(result) => result,
        Err(e) => {
            error!("解析 YOLO 輸出失敗: {}, 輸出內容: {}", e, stdout);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "無法解析辨識結果".into(),
                }),
            ));
        }
    };

    // 3. 熱量計算參數設定
    // ⚠️ 警告：這是一個預估值，假設 10 像素 = 1 公分。
    // 在真實應用中，你可能需要根據使用者上傳的圖片尺寸做正規化，或請使用者放硬幣當作比例尺。
    let pixel_to_cm = 0.1;
    let average_height_cm = 2.0;

    let mut total_calories = 0.0;
    let mut detected_items = Vec::new();

    // 4. 計算每個物件的熱量
    for det in yolo_result.detections {
        let x_min = det.bbox[0];
        let y_min = det.bbox[1];
        let x_max = det.bbox[2];
        let y_max = det.bbox[3];

        let width_cm = (x_max - x_min) * pixel_to_cm;
        let height_cm = (y_max - y_min) * pixel_to_cm;

        let volume_cm3 = width_cm * height_cm * average_height_cm;
        let weight_g = volume_cm3 * 1.0; // 假設密度為 1 g/cm3

        let cal_per_gram = match det.class_name.as_str() {
            "rice" => 1.3,
            "pork" => 2.5,
            "chicken" => 1.65,
            "leafy_veg" => 0.25,
            "mushroom" => 0.22,
            "apple" => 0.52,
            _ => 1.0,
        };

        let item_calories = weight_g * cal_per_gram;
        total_calories += item_calories;

        detected_items.push(FoodItem {
            class: det.class_name,
            confidence: (det.confidence * 100.0).round() / 100.0,
            estimated_weight_g: (weight_g * 10.0).round() / 10.0,
            calories: (item_calories * 10.0).round() / 10.0,
        });
    }

    Ok(Json(CalorieResponse {
        message: "辨識與熱量計算完成".into(),
        total_calories: (total_calories * 10.0).round() / 10.0,
        detected_items,
    }))
}
