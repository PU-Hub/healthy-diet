use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::Multipart;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs, process::Command, sync::Arc};
use tracing::error;
use uuid::Uuid;

use crate::{
    api::model::ErrorResponse,
    model::{AppState, ENVKey},
    utils::jwt::AuthUser,
};

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
    pub image_base64: Option<String>,
    // 🌟 讓前端也能馬上拿到評分與建議
    pub ai_score: i32,
    pub ai_comment: String,
}

#[derive(Serialize)]
pub struct FoodItem {
    pub class: String,
    pub confidence: f64,
    pub estimated_weight_g: f64,
    pub calories: f64,
}

// 🌟 定義接收 AI 回傳的 JSON 結構
#[derive(Deserialize, Debug)]
struct AiEvaluationFormat {
    score: i32,
    comment: String,
}

pub async fn yolo_handler(
    auth_user: AuthUser, // 現在需要 auth_user 來撈取資料跟存檔
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<CalorieResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 1. 取得使用者基本資料 (供 AI 評分參考)
    let user_profile = sqlx::query!(
        "SELECT nickname, height, weight, age, gender, taboo, disease FROM users WHERE id = $1",
        auth_user.user_id
    )
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
    let file_id = Uuid::new_v4().to_string();
    let input_path = format!("/app/uploads/{}.jpg", file_id);

    fs::create_dir_all("/app/uploads").ok();
    fs::write(&input_path, data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "伺服器儲存檔案失敗".into(),
            }),
        )
    })?;

    // 3. 呼叫 YOLO Python 腳本
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
        error!("YOLO 執行失敗: {}", String::from_utf8_lossy(&output.stderr));
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "YOLO 辨識過程出錯".into(),
            }),
        ));
    }

    let yolo_result: YoloScriptOutput =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).map_err(|e| {
            error!("解析 YOLO 失敗: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "解析辨識結果失敗".into(),
                }),
            )
        })?;

    // 4. 計算各類別面積與卡路里
    let pixel_to_cm = 0.05;
    let average_height_cm = 3.0;

    let mut total_calories = 0.0;
    let mut detected_items = Vec::new();

    // 🌟 定義 7 大分類的累加器
    let mut stats = std::collections::HashMap::new();
    let categories = [
        "grain",
        "protein_meat",
        "protein_bean",
        "vegetable",
        "fruit",
        "dairy",
        "nuts",
        "other",
    ];
    for cat in categories {
        stats.insert(cat, (0.0_f64, 0.0_f64)); // (Calories, Area)
    }

    for det in yolo_result.detections {
        let x_min = det.bbox[0];
        let y_min = det.bbox[1];
        let x_max = det.bbox[2];
        let y_max = det.bbox[3];

        let area_cm2 = (x_max - x_min) * (y_max - y_min) * (pixel_to_cm * pixel_to_cm);
        let volume_cm3 = area_cm2 * average_height_cm;

        // 根據新類別設定密度(g/cm3)與熱量(kcal/g)
        let (density, cal_per_gram, category_key) = match det.class_name.as_str() {
            "grain" => (1.0, 1.3, "grain"),               // 飯/麵
            "protein_meat" => (1.0, 2.5, "protein_meat"), // 肉類
            "protein_bean" => (1.0, 1.4, "protein_bean"), // 豆類/豆腐
            "vegetable" => (0.3, 0.25, "vegetable"),      // 蔬菜
            "fruit" => (0.8, 0.5, "fruit"),               // 水果
            "dairy" => (1.0, 0.6, "dairy"),               // 乳品
            "nuts" => (0.6, 6.0, "nuts"),                 // 堅果
            _ => (1.0, 1.0, "other"),                     // 其他未知
        };

        let raw_weight_g = volume_cm3 * density;
        let weight_g = raw_weight_g.clamp(10.0, 500.0); // 加上合理的上下限防呆
        let item_calories = weight_g * cal_per_gram;

        total_calories += item_calories;

        // 累加到分類統計中
        if let Some(entry) = stats.get_mut(category_key) {
            entry.0 += item_calories; // 累加卡路里
            entry.1 += area_cm2; // 累加面積
        }

        detected_items.push(FoodItem {
            class: det.class_name,
            confidence: (det.confidence * 100.0).round() / 100.0,
            estimated_weight_g: (weight_g * 10.0).round() / 10.0,
            calories: (item_calories * 10.0).round() / 10.0,
        });
    }

    // 5. 呼叫 Gemini 取得健康評分與短評
    let api_key = env::var(ENVKey::GEMINI_API_KEY).unwrap_or_default();
    let model_name =
        env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-1.5-flash-lite".to_string());
    let ai_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name, api_key
    );

    let taboo_str = user_profile.taboo.unwrap_or_default().join("、");
    let disease_str = user_profile.disease.unwrap_or_default().join("、");

    // 將系統提示詞分離，增強模型人設與任務指示
    let system_instruction = "你是一位嚴格但溫暖的專業營養師。請根據以下使用者的「身體狀況」與「本餐飲食辨識結果」，進行健康評估。\n\n\
        【任務要求】\n\
        1. 給予本餐健康評分(0~100)。蔬菜太少、總熱量過高(單餐>800)或觸犯禁忌請扣分；植物性蛋白質取代肉類可加分。\n\
        2. 給予一段簡短對話評語(嚴格限制在 45 個字以內)。\n\
        3. 必須且只能回傳以下 JSON 格式，不要加 ```json 標籤：\n\
        {\"score\": 85, \"comment\": \"你的蔬菜量不足，建議下餐多補充青菜喔！\"}";

    // 組合使用者的具體分析數據
    let user_prompt = format!(
        "【使用者資料】\n\
        年齡: {} / 性別: {} / 疾病史: {} / 飲食禁忌: {}\n\n\
        【本餐飲食分析】\n\
        總熱量: {:.1} kcal\n\
        - 全穀雜糧: {:.1} kcal\n\
        - 豆類蛋白質: {:.1} kcal\n\
        - 肉類蛋白質: {:.1} kcal\n\
        - 蔬菜: {:.1} kcal\n\
        - 水果: {:.1} kcal\n\
        - 乳品: {:.1} kcal\n\
        - 堅果: {:.1} kcal",
        user_profile
            .age
            .map(|v| v.to_string())
            .unwrap_or_else(|| "未提供".into()),
        user_profile.gender.unwrap_or_else(|| "未提供".into()),
        if disease_str.is_empty() {
            "無".to_string()
        } else {
            disease_str
        },
        if taboo_str.is_empty() {
            "無".to_string()
        } else {
            taboo_str
        },
        total_calories,
        stats["grain"].0,
        stats["protein_bean"].0,
        stats["protein_meat"].0,
        stats["vegetable"].0,
        stats["fruit"].0,
        stats["dairy"].0,
        stats["nuts"].0
    );

    // 採用與 consult 相同的 json! 巨集格式來組裝 Request
    let request_payload = json!({
        "system_instruction": { "parts": { "text": system_instruction } },
        "contents": [
            { "role": "user", "parts": [{ "text": user_prompt }] }
        ]
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&ai_url)
        .json(&request_payload)
        .send()
        .await
        .map_err(|e| {
            error!("Gemini API 發送錯誤: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "AI Sevice Unavailable".into(),
                }),
            )
        })?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        error!("Gemini API 回傳錯誤: {:?}", error_text);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "AI Service Error".into(),
            }),
        ));
    }

    let res_body: serde_json::Value = response.json().await.map_err(|e| {
        error!("Fail to parse Gemini Json: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "AI Response Parse Error".into(),
            }),
        )
    })?;

    let text_reply = res_body["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("{}")
        .to_string();

    let clean_json = text_reply
        .replace("```json", "")
        .replace("```", "")
        .trim()
        .to_string();

    // 預設值 (萬一 JSON 解析失敗仍能存檔)
    let mut ai_score = 60;
    let mut ai_comment = "飲食紀錄已儲存，繼續保持！".to_string();

    if let Ok(evaluation) = serde_json::from_str::<AiEvaluationFormat>(&clean_json) {
        ai_score = evaluation.score.clamp(0, 100);
        ai_comment = evaluation.comment;
    } else {
        error!("無法將 AI 回覆解析為結構: {}", clean_json);
    }

    // 6. 將所有數據存入資料庫 (diet_records)
    sqlx::query!(
        r#"
        INSERT INTO diet_records (
            user_id, total_calories,
            grain_calories, grain_area,
            protein_meat_calories, protein_meat_area,
            protein_bean_calories, protein_bean_area,
            vegetable_calories, vegetable_area,
            fruit_calories, fruit_area,
            dairy_calories, dairy_area,
            nuts_calories, nuts_area,
            ai_health_score, ai_evaluation
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18
        )
        "#,
        auth_user.user_id,
        total_calories,
        stats["grain"].0,
        stats["grain"].1,
        stats["protein_meat"].0,
        stats["protein_meat"].1,
        stats["protein_bean"].0,
        stats["protein_bean"].1,
        stats["vegetable"].0,
        stats["vegetable"].1,
        stats["fruit"].0,
        stats["fruit"].1,
        stats["dairy"].0,
        stats["dairy"].1,
        stats["nuts"].0,
        stats["nuts"].1,
        ai_score,
        ai_comment
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("飲食紀錄存檔失敗: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "資料庫儲存失敗".into(),
            }),
        )
    })?;

    // 7. 回傳結果給前端 (包含圖片與 AI 建議)
    let image_base64 = fs::read(&yolo_result.image_path)
        .ok()
        .map(|b| general_purpose::STANDARD.encode(b));

    Ok(Json(CalorieResponse {
        message: "辨識與熱量計算完成".into(),
        image_base64,
        total_calories: (total_calories * 10.0).round() / 10.0,
        detected_items,
        ai_score,
        ai_comment,
    }))
}
