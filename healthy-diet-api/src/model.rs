use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub struct APIRouter;

impl APIRouter {
    pub const PING: &'static str = "/ping";
    pub const DISOCRD_LOGIN: &'static str = "/auth/discord/login";
    pub const DISOCRD_CALLBACK: &'static str = "/auth/discord/callbck";
    pub const REGISTER: &'static str = "/auth/register";
    pub const LOGIN: &'static str = "/auth/login";
    pub const REFRESH_TOKEN: &'static str = "/auth/refresh";
    pub const PROFILE: &'static str = "/api/user/profile";
}
