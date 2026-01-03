use crate::{
    api::model::{AuthResponse, ErrorResponse, LoginPayload, User, UserProfile},
    model::AppState,
    utils::{hash::verify_password, jwt::sign_jwt},
};
use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;
use tracing::{error, info, instrument, warn};

#[instrument(skip(state, payload), fields(email = %payload.email))]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, nickname, avatar_url FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Database error during login: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        )
    })?;

    let user = match user {
        Some(u) => u,
        None => {
            warn!("Login failed: User not found");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid email or password".to_string(),
                }),
            ));
        }
    };

    let is_valid = verify_password(&payload.password, &user.password_hash).unwrap_or(false);

    if !is_valid {
        warn!("Login failed: Invalid password for user {}", user.id);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid email or password".to_string(),
            }),
        ));
    }

    let (token, refresh_token, expires_in) =
        sign_jwt(&user.id.to_string(), &user.email).map_err(|e| {
            error!("JWT generation error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to generate token".to_string(),
                }),
            )
        })?;

    info!("User logged in successfully: {}", user.id);

    Ok(Json(AuthResponse {
        token,
        refresh_token,
        expires_in,
        user: UserProfile {
            id: user.id.to_string(),
            email: user.email,
            nickname: user.nickname,
            avatar_url: user.avatar_url,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::{register_test_account, setup_db};

    #[tokio::test]
    async fn test_login_success() {
        let state = setup_db().await;

        let (email, password) = register_test_account(state.clone(), "login_ok".to_string()).await;

        let login_payload = LoginPayload {
            email: email.clone(),
            password,
        };

        let result = login_handler(State(state.clone()), Json(login_payload)).await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.user.email, email);
        assert!(!response.token.is_empty());
        assert!(!response.refresh_token.is_empty());
        assert!(response.expires_in > 0);

        let _ = sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&state.db)
            .await;
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let state = setup_db().await;

        let (email, _real_password) =
            register_test_account(state.clone(), "login_fail".to_string()).await;

        let login_payload = LoginPayload {
            email: email.clone(),
            password: "wrong_password_123".to_string(),
        };

        let result = login_handler(State(state.clone()), Json(login_payload)).await;

        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let _ = sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&state.db)
            .await;
    }
}
