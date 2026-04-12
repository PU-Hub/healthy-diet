use serde_json::Value;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub ai_prompt_config: Value,
}

pub struct APIRouter;

impl APIRouter {
    pub const PING: &'static str = "/ping";
    pub const DISOCRD_LOGIN: &'static str = "/auth/discord/login";
    pub const DISOCRD_CALLBACK: &'static str = "/auth/discord/callbck";
    pub const REGISTER: &'static str = "/auth/register";
    pub const LOGIN: &'static str = "/auth/login";
    pub const REFRESH_TOKEN: &'static str = "/auth/refresh";
    pub const PROFILE: &'static str = "/user/profile";
    pub const AI_CONSULT: &'static str = "/consult";
    pub const DIET: &'static str = "/diet";
    pub const HEALTH: &'static str = "/health";
    pub const DIET_RECORD: &'static str = "/diet_record";
    pub const DIET_IMAGE: &'static str = "/diet_image";
    pub const RECORD: &'static str = "/record";
    pub const TODAY_STATS: &'static str = "/today_stats";
}

pub struct ENVKey;

impl ENVKey {
    pub const PORT: &'static str = "PORT";
    pub const DATABASE_URL: &'static str = "DATABASE_URL";
    pub const DATABASE_URL_2: &'static str = "DATABASE_URL_2";
    pub const GEMINI_API_KEY: &'static str = "GEMINI_API_KEY";
    pub const JWT_SECRET: &'static str = "JWT_SECRET";
}

pub struct OutSideURL;

impl OutSideURL {
    pub const GEMINI_API_URL: &'static str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-flash-lite-preview:generateContent?key=";
}
