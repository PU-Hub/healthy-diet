use crate::{
    api::{
        agent_approve::approve_agent,
        chat_room::{
            get_chat_room_titles_handler, get_chat_rooms_handler, get_room_history_handler,
        },
        consult::consult_handler,
        diet::yolo_handler,
        diet_image::diet_image_handler,
        diet_record::diet_records_handler,
        generate_title::generate_room_title_handler,
        health::healthy_server_handler,
        login::login_handler,
        ping::ping_handler,
        proxy_chat::{proxy_agent_chat_handler, proxy_chat_check_handler},
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
    extract::DefaultBodyLimit,
    routing::{get, post, put},
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
        .route(APIRouter::PROXY_CHAT, post(proxy_agent_chat_handler))
        .route(APIRouter::PROXY_CHAT_CHECK, get(proxy_chat_check_handler))
        .route(APIRouter::AGENT_APPROVE, post(approve_agent))
        .route(APIRouter::CHAT_ROOM, get(get_chat_rooms_handler))
        .route(
            APIRouter::CHAT_ROOM_TITLES,
            get(get_chat_room_titles_handler),
        )
        .route(APIRouter::ROOM_HOSTROY, get(get_room_history_handler))
        .route(APIRouter::ROOM_TITLE, put(generate_room_title_handler))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
