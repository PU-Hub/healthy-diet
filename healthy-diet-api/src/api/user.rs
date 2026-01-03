use crate::{
    api::model::{AiConsultationRecord, ErrorResponse, UpdateProfilePayload, UserDetailResponse},
    model::AppState,
    utils::jwt::AuthUser,
};
use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;
use tracing::error;

pub async fn get_profile_handler(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query!(
        "SELECT id, email, nickname, avatar_url, height, weight, dietary_restrictions 
         FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("DB Error (Get Profile): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        )
    })?
    .ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "User not found".to_string(),
        }),
    ))?;

    let ai_records = sqlx::query!(
        "SELECT id, question, ai_response, created_at 
         FROM ai_consultations 
         WHERE user_id = $1 
         ORDER BY created_at DESC 
         LIMIT 10",
        auth_user.user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("DB Error (Get AI Records): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        )
    })?;

    let consultations = ai_records
        .into_iter()
        .map(|r| AiConsultationRecord {
            id: r.id,
            question: r.question,
            ai_response: r.ai_response,
            created_at: r.created_at.to_string(),
        })
        .collect();

    Ok(Json(UserDetailResponse {
        id: user.id.to_string(),
        email: user.email,
        nickname: user.nickname,
        avatar_url: user.avatar_url,
        height: user.height,
        weight: user.weight,
        dietary_restrictions: user.dietary_restrictions,
        ai_consultations: consultations,
    }))
}

pub async fn update_user_profile_handler(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateProfilePayload>,
) -> Result<Json<UserDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    let updated_user = sqlx::query!(
        r#"
        UPDATE users 
        SET 
            nickname = COALESCE($1, nickname),
            height = COALESCE($2, height),
            weight = COALESCE($3, weight),
            dietary_restrictions = COALESCE($4, dietary_restrictions)
        WHERE id = $5
        RETURNING id, email, nickname, avatar_url, height, weight, dietary_restrictions
        "#,
        payload.nickname,
        payload.height,
        payload.weight,
        payload.dietary_restrictions,
        auth_user.user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to update profile: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update profile".to_string(),
            }),
        )
    })?;

    Ok(Json(UserDetailResponse {
        id: updated_user.id.to_string(),
        email: updated_user.email,
        nickname: updated_user.nickname,
        avatar_url: updated_user.avatar_url,
        height: updated_user.height,
        weight: updated_user.weight,
        dietary_restrictions: updated_user.dietary_restrictions,
        ai_consultations: vec![],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::{register_test_account, setup_db};

    #[tokio::test]
    async fn test_get_profile_success() {
        let state = setup_db().await;

        let (email, _) = register_test_account(state.clone(), "profile_get".to_string()).await;

        let user = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_one(&state.db)
            .await
            .expect("Failed to fetch user");

        let auth_user = AuthUser {
            user_id: user.id,
            email: email.clone(),
        };

        let result = get_profile_handler(auth_user, State(state.clone())).await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.email, email);
        assert_eq!(response.ai_consultations.len(), 0);

        sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&state.db)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_update_profile_success() {
        let state = setup_db().await;

        let (email, _) = register_test_account(state.clone(), "profile_update".to_string()).await;
        let user = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_one(&state.db)
            .await
            .unwrap();

        let auth_user = AuthUser {
            user_id: user.id,
            email: email.clone(),
        };

        let payload = UpdateProfilePayload {
            nickname: Some("BigMuscle".to_string()),
            height: Some(180.5),
            weight: None,
            dietary_restrictions: None,
        };

        let result =
            update_user_profile_handler(auth_user, State(state.clone()), Json(payload)).await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.nickname, Some("BigMuscle".to_string()));
        assert_eq!(response.height, Some(180.5));
        assert_eq!(response.weight, None);

        let db_user = sqlx::query!("SELECT nickname, height FROM users WHERE id = $1", user.id)
            .fetch_one(&state.db)
            .await
            .unwrap();

        assert_eq!(db_user.nickname, Some("BigMuscle".to_string()));

        sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&state.db)
            .await
            .unwrap();
    }
}
