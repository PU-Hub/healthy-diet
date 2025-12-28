use crate::{
    common::{login::login_handler, register::register_handler},
    discord::login::{discord_callback, login_discord},
    model::{APIRouter, AppState},
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(async || "Connect Success!"))
        .route(APIRouter::DISOCRD_LOGIN, get(login_discord))
        .route(APIRouter::DISOCRD_CALLBACK, get(discord_callback))
        .route(APIRouter::REGISTER, post(register_handler))
        .route(APIRouter::LOGIN, post(login_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
