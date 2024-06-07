use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::tuning::Tuning;

pub async fn create_tuning(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_tuning): Json<Tuning>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Tuning,
        r#"
        INSERT INTO Tunings (car_id, tuning_name, tuning_date, tuning_description)
        VALUES ($1, $2, $3, $4)
        RETURNING tuning_id, car_id, tuning_name, tuning_date, tuning_description, created_at, updated_at
        "#,
        new_tuning.car_id,
        new_tuning.tuning_name,
        new_tuning.tuning_date,
        new_tuning.tuning_description
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(tuning) => (StatusCode::CREATED, Json(tuning)).into_response(),
        Err(e) => {
            eprintln!("Failed to create tuning: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_tunings(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Tuning, "SELECT * FROM Tunings")
        .fetch_all(&db_pool)
        .await
    {
        Ok(tunings) => (StatusCode::OK, Json(tunings)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch tunings: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_tuning(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Tuning, "SELECT * FROM Tunings WHERE tuning_id = $1", id)
        .fetch_one(&db_pool)
        .await
    {
        Ok(tuning) => (StatusCode::OK, Json(tuning)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch tuning: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn update_tuning(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
    Json(updated_tuning): Json<Tuning>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Tuning,
        r#"
        UPDATE Tunings
        SET car_id = $1, tuning_name = $2, tuning_date = $3, tuning_description = $4, updated_at = CURRENT_TIMESTAMP
        WHERE tuning_id = $5
        RETURNING tuning_id, car_id, tuning_name, tuning_date, tuning_description, created_at, updated_at
        "#,
        updated_tuning.car_id,
        updated_tuning.tuning_name,
        updated_tuning.tuning_date,
        updated_tuning.tuning_description,
        id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(tuning) => (StatusCode::OK, Json(tuning)).into_response(),
        Err(e) => {
            eprintln!("Failed to update tuning: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_tuning(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM Tunings WHERE tuning_id = $1", id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Failed to delete tuning: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
