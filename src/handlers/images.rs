use axum::{
    extract::{Multipart, Extension},
    response::IntoResponse,
    http::StatusCode,
};
use reqwest::Client;
use std::sync::Arc;
use std::env;
use tokio::sync::Mutex;
use crate::db::AppState;
use dotenv::dotenv;

pub async fn upload_image(
    Extension(_state): Extension<Arc<Mutex<AppState>>>, // db_poolを削除
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Cloudflare R2エンドポイントとAPIトークンの設定
	dotenv().ok();
    let r2_endpoint = env::var("R2_ENDPOINT_URL").expect("R2_ENDPOINT_URL must be set");
    let api_token = env::var("R2_API_TOKEN").expect("R2_API_TOKEN must be set");

    if let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        // Cloudflare R2にファイルをアップロード
        let client = Client::new();
        let response = client
            .put(r2_endpoint)
            .header("Authorization", format!("Bearer {}", api_token))
            .body(data)
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let image_url = response.text().await.unwrap();
            return (StatusCode::OK, image_url).into_response();
        } else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    StatusCode::BAD_REQUEST.into_response()
}
