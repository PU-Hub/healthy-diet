use crate::{
    api::{
        consult::consult_handler,
        diet::yolo_handler,
        diet_image::diet_image_handler,
        diet_record::diet_records_handler,
        health::healthy_server_handler,
        login::login_handler,
        ping::ping_handler,
        record::{record_visit_handler, weekly_stats_handler},
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
        .route("/ping", get(ping_handler))
        .route(APIRouter::HEALTH, get(healthy_server_handler))
        .route(APIRouter::DISOCRD_LOGIN, get(login_discord))
        .route(APIRouter::DISOCRD_CALLBACK, get(discord_callback))
        .route(APIRouter::REGISTER, post(register_handler))
        .route(APIRouter::LOGIN, post(login_handler))
        .route(APIRouter::REFRESH_TOKEN, post(refresh_handler))
        .route(
            APIRouter::PROFILE,
            get(get_profile_handler).put(update_user_profile_handler),
        )
        .route(APIRouter::AI_CONSULT, post(consult_handler))
        .route(APIRouter::DIET, post(yolo_handler))
        .route(APIRouter::DIET_RECORD, get(diet_records_handler))
        .route(APIRouter::DIET_IMAGE, post(diet_image_handler))
        .route(APIRouter::MONTH_STATS, get(weekly_stats_handler))
        .route(APIRouter::RECORD, post(record_visit_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
