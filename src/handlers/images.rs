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
    let access_key_id = env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set");
    let secret_access_key = env::var("R2_SECRET_ACCESS_KEY").expect("R2_SECRET_ACCESS_KEY must be set");

    println!("R2_ENDPOINT_URL: {}", r2_endpoint);
    println!("R2_ACCESS_KEY_ID: {}", access_key_id);
    println!("R2_SECRET_ACCESS_KEY: {}", secret_access_key);

    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(e) => {
            println!("Error reading field: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    } {
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
            .header("x-amz-date", chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string())
            .header("Authorization", format!("AWS {}:{}", access_key_id, secret_access_key))
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
