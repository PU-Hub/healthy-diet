use axum::{Router, routing::get};

use tower_http::trace::TraceLayer;

use crate::{common::test::test, nhentai::search::nhentai_search};

pub fn create_app() -> Router {
    Router::new()
        .route("/", async || "Connect Success!")
        .layer(TraceLayer::new_for_http())
}
