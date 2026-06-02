use crate::{
    api::{
        admin::{
            admin_announcements_handler, admin_me_handler, admin_route_controls_handler,
            admin_user_detail_handler, admin_users_handler, archive_announcement_handler,
            create_agent_admin_token_handler, create_announcement_handler,
            publish_announcement_handler, update_announcement_handler,
            update_route_control_handler,
        },
        agent_approve::approve_agent,
        agent_content::{
            news_detail_handler, news_files_handler, news_list_handler, news_sync_handler,
            rag_search_get_handler, rag_search_post_handler,
        },
        announcement::current_announcement_handler,
        chat_room::{
            get_chat_room_titles_handler, get_chat_rooms_handler,
            get_room_history_by_index_handler, get_room_history_handler,
        },
        consult::consult_handler,
        diet::yolo_handler,
        diet_image::diet_image_handler,
        diet_record::diet_records_handler,
        gemma4::gemma4_health_handler,
        health::healthy_server_handler,
        login::{admin_login_handler, login_handler},
        ping::ping_handler,
        proxy_chat::{proxy_agent_chat_handler, proxy_chat_check_handler},
        rag_document::{
            admin_rag_delete_handler, admin_rag_document_detail_handler,
            admin_rag_document_file_handler, admin_rag_document_preview_handler,
            admin_rag_documents_handler, admin_rag_reindex_handler, admin_rag_upload_handler,
            public_rag_document_file_handler, public_rag_document_preview_handler,
        },
        record::{record_visit_handler, weekly_stats_handler},
        refresh::refresh_handler,
        register::register_handler,
        user::{get_profile_handler, update_user_profile_handler},
    },
    discord::login::{discord_callback, login_discord},
    model::{APIRouter, AppState},
    utils::jwt::require_admin_middleware,
    utils::route_control::{RouteControlGuardState, require_route_enabled_middleware},
};
use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_app(state: Arc<AppState>) -> Router {
    let admin_router = Router::new()
        .route(APIRouter::ADMIN_ME, get(admin_me_handler))
        .route(
            APIRouter::ADMIN_AGENT_TOKEN,
            post(create_agent_admin_token_handler),
        )
        .route(APIRouter::ADMIN_USERS, get(admin_users_handler))
        .route(APIRouter::ADMIN_USER_DETAIL, get(admin_user_detail_handler))
        .route(
            APIRouter::ADMIN_ROUTE_CONTROLS,
            get(admin_route_controls_handler),
        )
        .route(
            APIRouter::ADMIN_ROUTE_CONTROL_DETAIL,
            axum::routing::patch(update_route_control_handler),
        )
        .route(
            APIRouter::ADMIN_ANNOUNCEMENTS,
            get(admin_announcements_handler).post(create_announcement_handler),
        )
        .route(
            APIRouter::ADMIN_ANNOUNCEMENT_DETAIL,
            axum::routing::patch(update_announcement_handler),
        )
        .route(
            APIRouter::ADMIN_ANNOUNCEMENT_PUBLISH,
            post(publish_announcement_handler),
        )
        .route(
            APIRouter::ADMIN_ANNOUNCEMENT_ARCHIVE,
            post(archive_announcement_handler),
        )
        .route(
            APIRouter::ADMIN_RAG_DOCUMENTS,
            get(admin_rag_documents_handler).post(admin_rag_upload_handler),
        )
        .route(
            APIRouter::ADMIN_RAG_DOCUMENT_DETAIL,
            get(admin_rag_document_detail_handler).delete(admin_rag_delete_handler),
        )
        .route(
            APIRouter::ADMIN_RAG_DOCUMENT_REINDEX,
            post(admin_rag_reindex_handler),
        )
        .route(
            APIRouter::ADMIN_RAG_DOCUMENT_FILE,
            get(admin_rag_document_file_handler),
        )
        .route(
            APIRouter::ADMIN_RAG_DOCUMENT_PREVIEW,
            get(admin_rag_document_preview_handler),
        )
        .route_layer(middleware::from_fn(require_admin_middleware));

    Router::new()
        .route("/", get(async || "Connect Success!"))
        .route("/ping", get(ping_handler))
        .route(APIRouter::HEALTH, get(healthy_server_handler))
        .route(APIRouter::DISOCRD_LOGIN, get(login_discord))
        .route(APIRouter::DISOCRD_CALLBACK, get(discord_callback))
        .route(APIRouter::REGISTER, post(register_handler))
        .route(APIRouter::LOGIN, post(login_handler))
        .route(APIRouter::ADMIN_LOGIN, post(admin_login_handler))
        .route(APIRouter::REFRESH_TOKEN, post(refresh_handler))
        .route(
            APIRouter::PROFILE,
            get(get_profile_handler).put(update_user_profile_handler),
        )
        .route(
            APIRouter::AI_CONSULT,
            post(consult_handler).route_layer(middleware::from_fn_with_state(
                RouteControlGuardState {
                    app_state: state.clone(),
                    route_key: "consult",
                },
                require_route_enabled_middleware,
            )),
        )
        .route(
            APIRouter::DIET,
            post(yolo_handler).route_layer(middleware::from_fn_with_state(
                RouteControlGuardState {
                    app_state: state.clone(),
                    route_key: "diet",
                },
                require_route_enabled_middleware,
            )),
        )
        .route(APIRouter::DIET_RECORD, get(diet_records_handler))
        .route(
            APIRouter::DIET_IMAGE,
            post(diet_image_handler).route_layer(middleware::from_fn_with_state(
                RouteControlGuardState {
                    app_state: state.clone(),
                    route_key: "diet_image",
                },
                require_route_enabled_middleware,
            )),
        )
        .route(APIRouter::MONTH_STATS, get(weekly_stats_handler))
        .route(APIRouter::GEMMA4_HEALTH, get(gemma4_health_handler))
        .route(APIRouter::RECORD, post(record_visit_handler))
        .route(APIRouter::NEWS_SYNC, post(news_sync_handler))
        .route(APIRouter::NEWS, get(news_list_handler))
        .route(APIRouter::NEWS_DETAIL, get(news_detail_handler))
        .route(APIRouter::NEWS_FILES, get(news_files_handler))
        .route(
            APIRouter::RAG_SEARCH,
            get(rag_search_get_handler).post(rag_search_post_handler),
        )
        .route(
            APIRouter::PROXY_CHAT,
            post(proxy_agent_chat_handler).route_layer(middleware::from_fn_with_state(
                RouteControlGuardState {
                    app_state: state.clone(),
                    route_key: "proxy_chat",
                },
                require_route_enabled_middleware,
            )),
        )
        .route(APIRouter::PROXY_CHAT_CHECK, get(proxy_chat_check_handler))
        .route(
            APIRouter::RAG_SOURCE_FILE,
            get(public_rag_document_file_handler),
        )
        .route(
            APIRouter::RAG_SOURCE_PREVIEW,
            get(public_rag_document_preview_handler),
        )
        .route(
            APIRouter::ANNOUNCEMENTS_CURRENT,
            get(current_announcement_handler),
        )
        .route(APIRouter::AGENT_APPROVE, post(approve_agent))
        .route(APIRouter::CHAT_ROOM, get(get_chat_rooms_handler))
        .route(
            APIRouter::CHAT_ROOM_TITLES,
            get(get_chat_room_titles_handler),
        )
        .route(APIRouter::ROOM_HOSTROY, get(get_room_history_handler))
        .route(
            APIRouter::ROOM_HISTORY_BY_INDEX,
            get(get_room_history_by_index_handler),
        )
        .nest(APIRouter::ADMIN, admin_router)
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
