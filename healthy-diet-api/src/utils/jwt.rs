use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::api::model::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

pub fn sign_jwt(
    user_id: &str,
    email: &str,
) -> Result<(String, String, usize), jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let access_expires_in = 3600;
    let refresh_expires_in = 86400 * 7;
    let access_exp = now + access_expires_in;
    let refresh_exp = now + refresh_expires_in;

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = EncodingKey::from_secret(secret.as_bytes());

    let access_claims = Claims {
        sub: user_id.to_owned(),
        email: email.to_owned(),
        exp: access_exp,
        iat: now,
        token_type: "access".to_string(),
    };
    let access_token = encode(&Header::default(), &access_claims, &key)?;

    let refresh_claims = Claims {
        sub: user_id.to_owned(),
        email: email.to_owned(),
        exp: refresh_exp,
        iat: now,
        token_type: "refresh".to_string(),
    };
    let refresh_token = encode(&Header::default(), &refresh_claims, &key)?;

    Ok((access_token, refresh_token, access_expires_in))
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_bytes());
    let token_data = decode::<Claims>(token, &key, &Validation::default())?;
    Ok(token_data.claims)
}

pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub email: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Missing Bearer Token".to_string(),
                    }),
                )
                    .into_response()
            })?;

        let claims = decode_jwt(bearer.token()).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid Token".to_string(),
                }),
            )
                .into_response()
        })?;

        let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid User ID format".to_string(),
                }),
            )
                .into_response()
        })?;

        Ok(AuthUser {
            user_id,
            email: claims.email,
        })
    }
}
