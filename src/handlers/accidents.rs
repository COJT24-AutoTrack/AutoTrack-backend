use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::accident::Accident;

pub async fn create_accident(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_accident): Json<Accident>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Accident,
        r#"
        INSERT INTO Accidents (car_id, accident_date, accident_description)
        VALUES ($1, $2, $3)
        RETURNING accident_id, car_id, accident_date, accident_description, created_at, updated_at
        "#,
        new_accident.car_id,
        new_accident.accident_date,
        new_accident.accident_description
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(accident) => (StatusCode::CREATED, Json(accident)).into_response(),
        Err(e) => {
            eprintln!("Failed to create accident: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_accidents(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Accident, "SELECT * FROM Accidents")
        .fetch_all(&db_pool)
        .await
    {
        Ok(accidents) => (StatusCode::OK, Json(accidents)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch accidents: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_accident(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Accident, "SELECT * FROM Accidents WHERE accident_id = $1", id)
        .fetch_one(&db_pool)
        .await
    {
        Ok(accident) => (StatusCode::OK, Json(accident)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch accident: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn update_accident(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
    Json(updated_accident): Json<Accident>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Accident,
        r#"
        UPDATE Accidents
        SET car_id = $1, accident_date = $2, accident_description = $3, updated_at = CURRENT_TIMESTAMP
        WHERE accident_id = $4
        RETURNING accident_id, car_id, accident_date, accident_description, created_at, updated_at
        "#,
        updated_accident.car_id,
        updated_accident.accident_date,
        updated_accident.accident_description,
        id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(accident) => (StatusCode::OK, Json(accident)).into_response(),
        Err(e) => {
            eprintln!("Failed to update accident: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_accident(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM Accidents WHERE accident_id = $1", id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Failed to delete accident: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
