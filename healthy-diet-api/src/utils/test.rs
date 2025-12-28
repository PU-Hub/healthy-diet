#![cfg(test)]
use std::{env, sync::Arc};

use axum::{Json, extract::State};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::{
    common::{model::RegisterPayload, register::register_handler},
    model::AppState,
};

pub async fn setup_db() -> Arc<AppState> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");
    Arc::new(AppState { db: pool })
}

pub async fn register_test_account(state: Arc<AppState>, email_name: String) -> (String, String) {
    let email = format!("{}_{}@example.com", email_name, Uuid::new_v4());

    let password = format!("{}_{}", email_name, Uuid::new_v4());

    let reg_payload = RegisterPayload {
        email: email.clone(),
        password: password.to_string(),
        nickname: None,
        avatar_url: None,
    };
    let _ = register_handler(State(state.clone()), Json(reg_payload))
        .await
        .expect("register fail");

    info!(
        "register account success! email: {}, password: {}",
        email, password
    );
    (email, password)
}
