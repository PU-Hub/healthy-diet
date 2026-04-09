use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::io::{self, Write};
use chrono::Local;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
  gender: String,
  height: f64,
  weight: f64,
  age: f64,
  taboo: Vec<String>,
  disease: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("❌ 請在 .env 設定 KEY");

    // 1. 讀取 AIPrompt.json 設定
    let config_str = fs::read_to_string("AIPrompt.json").expect("❌ 找不到 AIPrompt.json");
    let config: Value = serde_json::from_str(&config_str).expect("❌ JSON 格式錯誤");


    // 2. 組裝 XML格式的system Instruction
    let system_prompt = build_xml_system_prompt(&config);


    // 3. 初始化 API 與 對話紀錄
    let client = Client::new();
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";
    let api_url = format!("{}?key={}", url, api_key);

    let mut history: Vec<Value> = Vec::new(); // 初始化對話紀錄

    println!("🚀 暖心營養師已就位！(輸入 exit 離開)");

    // 4. 對話主迴圈
    loop {
        print!("\n你：");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            println!("👋 下次見！要記得乖乖吃飯喔。");
            break;
        }

        if input.is_empty() { continue; }

        // 加入使用者訊息到紀錄
        let current_user = "people1";
        let (target_tdee, diseases, taboo) = user_disease(current_user);
        let today_cal=todaycalories();
        let remaining = target_tdee - today_cal;
        let prompt_with_context = format!(
          "<user_context>\n
          <health_profile>\n
          <diseases>{:?}</diseases>\n
          <taboo>{:?}</taboo>\n
          </health_profile>\n
          <daily_stats>\n
          <target_caloeies>{}</target_caloeies>\n
          <consumed_calories>{}</consumed_calories>\n
          <remaining_calories>{}</remaining_calories>\n
          </daily_stats>\n
          </user_context>\n\n
          <user_input>\n\"\"\"{}\"\"\"\n</user_input>",
          diseases, taboo, target_tdee, today_cal, remaining, input
        );
        history.push(json!({ "role": "user", "parts": [{ "text": prompt_with_context }] }));

        // 準備傳送給 API 的資料
        let payload = json!({
            "system_instruction": { "parts": { "text": system_prompt } },
            "contents": history,
            "generationConfig": {
                "responseMimeType": "application/json",
                "responseJsonSchema": config["output_schema"]
            }
        });

        // 發送請求
        let res = client.post(&api_url).json(&payload).send().await?;
        let res_body: Value = res.json().await?;

        // 5. 解析 AI 回覆
        if let Some(candidate) = res_body["candidates"].get(0) {
            if let Some(text) = candidate["content"]["parts"][0]["text"].as_str() {

                // 解析 AI 吐回來的 JSON 字串
                match serde_json::from_str::<Value>(text) {
                    Ok(ai_data) => {
                        let short = ai_data["short_reply"].as_str().unwrap_or("...");
                        let detailed = ai_data["detailed_report"].as_str().unwrap_or("無詳細資料");
                        let cal = ai_data["calories"].as_i64().unwrap_or(0);
                        let is_sport = ai_data["sport"].as_bool().unwrap_or(false);

                        // 印出給長輩看的話
                        println!("\n[暖心營養師]：{}", short);
                        println!("==========================");
                        println!("本餐大概:{}大卡,使用者有運動嗎:{}", cal, is_sport);
                        println!("詳細報告：{}", detailed);
                        println!("==========================");

                        save_to_database(cal, is_sport, detailed);

                        // 把對話紀錄存入記憶
                        history.push(json!({ "role": "model", "parts": [{ "text": text }] }));
                    },
                    Err(e) => println!("❌ JSON 解析失敗：{}，原始內容：{}", e, text),
                }

            } else {
                println!("⚠️ AI 停止回覆，原因：{:?}", candidate["finishReason"]);
            }
        } else if let Some(error) = res_body["error"].as_object() {
            println!("❌ API 報錯：{}", error["message"].as_str().unwrap_or("未知錯誤"));
        } else {
            println!("❌ 發生未知錯誤。");
        }

        // 6. 限制對話記憶長度 (保留最近 5 輪來回)
        if history.len() > 10 {
            history.drain(0..2);
        }
    }

    Ok(())
}

fn build_xml_system_prompt(config: &Value) -> String {
  let tasks = config["tasks"].as_array()
          .map(|a| a.iter().map(|v| format!("  <task>{}</task>", v.as_str().unwrap_or(""))).collect::<Vec<_>>().join("\n"))
          .unwrap_or_default();

      let styles = config["speaking_styles"].as_array()
          .map(|a| a.iter().map(|v| format!("  <style>{}</style>", v.as_str().unwrap_or(""))).collect::<Vec<_>>().join("\n"))
          .unwrap_or_default();

      let words = config["forbidden_words"].as_array()
          .map(|a| a.iter().map(|v| v.as_str().unwrap_or("")).collect::<Vec<_>>().join(", "))
          .unwrap_or_default();

  let examples = config["examples"].as_array().map(|a|{
    a.iter().map(|ex|{
      let input = ex["user_input"].as_str().or(ex["user_input"].as_str()).unwrap_or("");
      let output = serde_json::to_string(&ex["AI_output"]).unwrap_or_default();
      format!(" <example>\n <input>{}</input>\n  <output>{}</output>\n </example>", input , ex["AI_output"])
    }
    ).collect::<Vec<_>>().join("\n")
  }
    ).unwrap_or_default();
  format!(
    "<system_prompt>
    <role>{}</role>
    <instructions>
    {}
    </instructions>
    <style_guidelines>
    {}
    </style_guidelines>
    <negative_constraints>絕對嚴禁使用詞彙：{}</negative_constraints>
    <few_shot_learning>
    {}
    </few_shot_learning>
    <note>詳細報告請務必使用 Markdown 排版，確保家屬易於閱讀。</note>
    </system_prompt>",
            config["identity"].as_str().unwrap_or(""), tasks, styles, words, examples

  )
}


////////

fn save_to_database(cal: i64,is_sport:bool,detailed:&str){
let file_path = "health_database.json";
let new_record = json!({
  "timestamp": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
  "calories": cal,
  "is_sport": is_sport,
  "detailed": detailed
});

let mut all_records: Vec<Value> = if let Ok(content) = fs::read_to_string(file_path){
  serde_json::from_str(&content).unwrap_or_else(|_| vec![])
} else {
  vec![]
};
all_records.push(new_record);

if let Ok(mut file)= fs::File::create(file_path)
{
  let _ = file.write_all(serde_json::to_string_pretty(&all_records).unwrap_or_default().as_bytes());
}
}



fn todaycalories()->i64{
  let file_path ="health_database.json";
  let now_date = Local::now().format("%Y-%m-%d").to_string();

  if let Ok(contents) = fs::read_to_string(file_path) {
      let records: Vec<serde_json::Value> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);

      records.iter()
        .filter(|r|r["timestamp"].as_str().unwrap_or("").starts_with(&now_date))
        .map(|r|r["calories"].as_i64().unwrap_or(0))
        .sum()
    }
    else {
      0
    }
}

fn activity_level() -> f64 {
  let file_path = "health_database";
  if let Ok(contents) = fs::read_to_string(file_path) {
    let records:Vec<Value>=serde_json::from_str(&contents).unwrap_or_else(|_|vec![]);

    let exercise_count = records.iter().rev().take(21)
      .filter(|r|r["sport"].as_bool().unwrap_or(false))
      .count();

    if exercise_count >= 1 {
      1.375
    } else {
      1.2
    }
  } else {
    1.2
  }
}

fn user_disease(user_id: &str) -> (i64, Vec<String>, Vec<String>) {
  let mutiplier = activity_level();
  let content = fs::read_to_string("people.json").unwrap_or_else(|_|"{}".to_string());
  let all_people: Value = serde_json::from_str(&content).unwrap_or_default();

  if let Some(person_data) = all_people.get(user_id){
    if let Ok(person)=serde_json::from_value::<Person>(person_data.clone())
    {
      let bmr = if person.gender=="male"{
        66.0+(13.7*person.weight)+(5.0*person.height)-(6.8*person.age)
      } else {
        665.0+(9.6*person.weight)+(1.8*person.height)-(4.7*person.age)
      };
      return ((bmr*mutiplier)as i64, person.disease, person.taboo);
    }
  }
  (2000, vec![],vec![])
}
