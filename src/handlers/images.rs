use axum::{
    extract::multipart::Multipart,
    response::{Json, Response},
};
use cloudflare_r2_rs::r2::R2Manager;
use dotenv::dotenv;
use std::env;
use tracing::{info, error};

pub async fn upload_image(mut payload: Multipart) -> Result<Json<String>, Response> {
    dotenv().ok(); // 環境変数をロード

    let bucket = env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");
    let endpoint = env::var("R2_ENDPOINT_URL").expect("R2_ENDPOINT_URL must be set");
    let client_id = env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set");
    let secret = env::var("R2_SECRET_ACCESS_KEY").expect("R2_SECRET_ACCESS_KEY must be set");

    let r2_manager = R2Manager::new(&bucket, &endpoint, &client_id, &secret).await;

    while let Some(field) = payload.next_field().await.map_err(|e| {
        error!("Error reading field: {}", e);
        Response::builder()
            .status(400)
            .body(format!("Error reading field: {}", e).into())
            .unwrap()
    })? {
        if let Some(file_name) = field.file_name() {
            let file_name = file_name.to_owned();
            let content = field.bytes().await.map_err(|e| {
                error!("Error reading file content: {}", e);
                Response::builder()
                    .status(400)
                    .body(format!("Error reading file content: {}", e).into())
                    .unwrap()
            })?;
            let key = format!("images/{}", file_name);
            r2_manager.upload(&key, &content, None, Some("image/jpeg")).await;

            let url = format!("https://r2.autotrack.work/images/{}", file_name);
            info!("File uploaded successfully: {}", url);
            return Ok(Json(url));
        }
    }

    error!("No file uploaded");
    Err(axum::response::Response::builder()
        .status(400)
        .body("Error: No file uploaded".into())
        .unwrap())
}