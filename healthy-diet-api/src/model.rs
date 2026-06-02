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
    pub const ADMIN_LOGIN: &'static str = "/auth/admin/login";
    pub const REFRESH_TOKEN: &'static str = "/auth/refresh";
    pub const ADMIN: &'static str = "/admin";
    pub const ADMIN_ME: &'static str = "/me";
    pub const ADMIN_USERS: &'static str = "/users";
    pub const ADMIN_USER_DETAIL: &'static str = "/users/{user_id}";
    pub const ADMIN_ROUTE_CONTROLS: &'static str = "/route-controls";
    pub const ADMIN_ROUTE_CONTROL_DETAIL: &'static str = "/route-controls/{route_key}";
    pub const ADMIN_ANNOUNCEMENTS: &'static str = "/announcements";
    pub const ADMIN_ANNOUNCEMENT_DETAIL: &'static str = "/announcements/{id}";
    pub const ADMIN_ANNOUNCEMENT_PUBLISH: &'static str = "/announcements/{id}/publish";
    pub const ADMIN_ANNOUNCEMENT_ARCHIVE: &'static str = "/announcements/{id}/archive";
    pub const ADMIN_RAG_DOCUMENTS: &'static str = "/rag/documents";
    pub const ADMIN_RAG_DOCUMENT_DETAIL: &'static str = "/rag/documents/{id}";
    pub const ADMIN_RAG_DOCUMENT_REINDEX: &'static str = "/rag/documents/{id}/reindex";
    pub const ADMIN_RAG_DOCUMENT_FILE: &'static str = "/rag/documents/{id}/file";
    pub const ADMIN_RAG_DOCUMENT_PREVIEW: &'static str = "/rag/documents/{id}/preview";
    pub const ADMIN_AGENT_TOKEN: &'static str = "/agent-token";
    pub const NEWS_SYNC: &'static str = "/api/news/sync";
    pub const NEWS: &'static str = "/api/news";
    pub const NEWS_DETAIL: &'static str = "/api/news/{id}";
    pub const NEWS_FILES: &'static str = "/api/news-files";
    pub const RAG_SEARCH: &'static str = "/api/rag/search";
    pub const ANNOUNCEMENTS_CURRENT: &'static str = "/announcements/current";
    pub const RAG_SOURCE_FILE: &'static str = "/rag/sources/{id}/file";
    pub const RAG_SOURCE_PREVIEW: &'static str = "/rag/sources/{id}/preview";
    pub const PROFILE: &'static str = "/user/profile";
    pub const AI_CONSULT: &'static str = "/consult";
    pub const DIET: &'static str = "/diet";
    pub const HEALTH: &'static str = "/health";
    pub const DIET_RECORD: &'static str = "/diet_record";
    pub const DIET_IMAGE: &'static str = "/diet_image";
    pub const RECORD: &'static str = "/record";
    pub const MONTH_STATS: &'static str = "/month_stats";
    pub const GEMMA4_HEALTH: &'static str = "/gemma4/health";
    pub const PROXY_CHAT: &'static str = "/chat";
    pub const PROXY_CHAT_CHECK: &'static str = "/proxy_chat_check";
    pub const AGENT_APPROVE: &'static str = "/approve";
    pub const CHAT_ROOM: &'static str = "/chat_rooms";
    pub const CHAT_ROOM_TITLES: &'static str = "/chat_room_titles";
    pub const ROOM_HOSTROY: &'static str = "/room_history/{room_id}";
    pub const ROOM_HISTORY_BY_INDEX: &'static str = "/room_history/{room_id}/index/{index}";
}

pub struct ENVKey;

impl ENVKey {
    pub const PORT: &'static str = "PORT";
    pub const DATABASE_URL: &'static str = "DATABASE_URL";
    pub const DATABASE_URL_2: &'static str = "DATABASE_URL_2";
    pub const GEMINI_API_KEY: &'static str = "GEMINI_API_KEY";
    pub const JWT_SECRET: &'static str = "JWT_SECRET";
    pub const AGENT_API_URL: &'static str = "AGENT_API_URL";
    pub const CHAT_IMAGE_UPLOAD_DIR: &'static str = "CHAT_IMAGE_UPLOAD_DIR";
    pub const RAG_DOCS_ROOT: &'static str = "RAG_DOCS_ROOT";
}

pub struct OutSideURL;

impl OutSideURL {
    pub const GEMINI_API_URL: &'static str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-flash-lite-preview:generateContent?key=";
}
