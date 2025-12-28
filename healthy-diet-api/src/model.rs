use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub struct APIRouter;

impl APIRouter {
    pub const PING: &'static str = "/api/ping";
    pub const DISOCRD_LOGIN: &'static str = "/api/auth/discord/login";
    pub const DISOCRD_CALLBACK: &'static str = "/api/auth/discord/callbck";
    pub const REGISTER: &'static str = "/api/auth/register";
    pub const LOGIN: &'static str = "/api/auth/login";
}
