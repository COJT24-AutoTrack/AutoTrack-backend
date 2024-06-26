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
    dotenv().ok();
    let r2_endpoint = env::var("R2_ENDPOINT_URL").expect("R2_ENDPOINT_URL must be set");
    let api_token = env::var("R2_API_TOKEN").expect("R2_API_TOKEN must be set");

    println!("R2_ENDPOINT_URL: {}", r2_endpoint);
    println!("R2_API_TOKEN: {}", api_token);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("unknown").to_string();
        println!("Uploading file: {}", filename);

        let data = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Failed to read file bytes: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        // Cloudflare R2にファイルをアップロード
        let client = Client::new();
        let response = match client
            .put(&r2_endpoint)
            .header("Authorization", format!("Bearer {}", api_token))
            .body(data)
            .send()
            .await {
            Ok(res) => res,
            Err(e) => {
                println!("Error sending request to R2: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        if response.status().is_success() {
            let image_url = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    println!("Failed to read response text: {:?}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };
            println!("Image uploaded successfully: {}", image_url);
            return (StatusCode::OK, image_url).into_response();
        } else {
            println!("Failed to upload image: {:?}", response.status());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    StatusCode::BAD_REQUEST.into_response()
}
