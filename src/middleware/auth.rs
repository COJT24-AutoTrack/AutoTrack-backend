use std::sync::Arc;
use std::time::Duration;

use crate::state::AppState;
use anyhow::Context as _;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{
    decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, TokenData, Validation,
};
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub email_verified: bool,
}

const JWK_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

pub struct AppError {
    status_code: StatusCode,
    code: String,
    message: String,
}

impl AppError {
    pub fn new(status_code: StatusCode, code: String, message: String) -> Self {
        Self {
            status_code,
            code,
            message,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!({
                "code": self.code,
                "message": self.message,
            })),
        )
            .into_response()
    }
}

pub async fn jwt_auth(
    State(state): State<Arc<Mutex<AppState>>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    tracing::info!("ユーザー認証を行います");

    let authorization_header = request.headers().get("Authorization").ok_or(AppError::new(
        StatusCode::UNAUTHORIZED,
        "auth/missing-authorization-header".to_string(),
        "Authorization header is missing.".to_string(),
    ))?;
    let authorization = authorization_header.to_str().map_err(|e| {
        AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/invalid-authorization-header".to_string(),
            e.to_string(),
        )
    })?;

    if !authorization.starts_with("Bearer ") {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/invalid-authorization-header".to_string(),
            "Authorization header is invalid. It should start with 'Bearer'.".to_string(),
        ));
    }

    let jwt_token = authorization.trim_start_matches("Bearer ");

    // Mutex をロックして AppState の内容にアクセス
    let app_state = state.lock().await;
    let token = match verify_id_token(jwt_token, &app_state.firebase_project_id).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Failed to verify: {e}");
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "auth/invalid-token".to_string(),
                e.to_string(),
            ));
        }
    };

    // メールが認証されているか確認
    if app_state.require_email_verification && !token.claims.email_verified {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/email-not-verified".to_string(),
            "Email is not verified.".to_string(),
        ));
    }

    // Mutex のロックを解除
    drop(app_state);

    request.extensions_mut().insert(token.claims.sub.clone());

    tracing::info!("ユーザー認証が完了しました");
    Ok(next.run(request).await)
}

pub(crate) async fn verify_id_token(
    token: &str,
    firebase_project_id: &str,
) -> anyhow::Result<TokenData<Claims>> {
    let header = decode_header(token)?;
    let kid = header.kid.context("No key ID found in JWT header")?;

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(60))
        .build()
        .context("Failed to create HTTP client")?;
    let jwks: JwkSet = client.get(JWK_URL).send().await?.json().await?;

    let jwk = jwks.find(&kid).context("Unknown key ID")?;
    let key = DecodingKey::from_jwk(jwk)?;

    let mut validation = Validation::new(Algorithm::RS256);

    validation.validate_exp = true;
    validation.validate_nbf = false;
    validation.set_audience(&[firebase_project_id]);
    validation.set_issuer(&[format!(
        "https://securetoken.google.com/{}",
        firebase_project_id
    )]);
    validation.sub = None;

    let data = decode(token, &key, &validation).context("Failed to validate JWT")?;

    Ok(data)
}
