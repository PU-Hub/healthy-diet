use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email(message = "Email formate not correct"))]
    pub email: String,

    #[validate(length(min = 6, message = "paasword must great than 6 characters"))]
    pub password: String,

    #[validate(length(
        min = 2,
        max = 12,
        message = "length of nicname must between 2 and 12 characters"
    ))]
    pub nickname: Option<String>,

    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: usize,
    pub user: UserProfile,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenPayload {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfilePayload {
    pub nickname: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub dietary_restrictions: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AiConsultationRecord {
    pub id: Uuid,
    pub question: String,
    pub ai_response: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailResponse {
    pub id: String,
    pub email: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub dietary_restrictions: Option<String>,
    pub ai_consultations: Vec<AiConsultationRecord>,
}
