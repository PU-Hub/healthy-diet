use crate::{
    api::{
        login::login_handler,
        refresh::refresh_handler,
        register::register_handler,
        user::{get_profile_handler, update_user_profile_handler},
    },
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
        .route(APIRouter::REFRESH_TOKEN, post(refresh_handler))
        .route(
            APIRouter::PROFILE,
            get(get_profile_handler).put(update_user_profile_handler),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
