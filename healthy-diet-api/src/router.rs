use axum::{Router, routing::get};

use tower_http::trace::TraceLayer;

use crate::discord::login::{discord_callback, login_discord};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(async || "Connect Success!"))
        .route("/api/auth/discord/login", get(login_discord))
        .route("/api/auth/discord/callback", get(discord_callback))
        .layer(TraceLayer::new_for_http())
}
