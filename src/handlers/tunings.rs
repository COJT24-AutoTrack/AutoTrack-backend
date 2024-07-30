use crate::models::tuning::Tuning;
use crate::state::AppState;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{query, query_as};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_tuning(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_tuning): Json<Tuning>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO Tunings (car_id, tuning_name, tuning_date, tuning_description) VALUES (?, ?, ?, ?)",
        new_tuning.car_id,
        new_tuning.tuning_name,
        new_tuning.tuning_date,
        new_tuning.tuning_description
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                Tuning,
                "SELECT tuning_id, car_id, tuning_name, tuning_date, tuning_description, created_at, updated_at FROM Tunings WHERE car_id = ?",
                new_tuning.car_id
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(tuning) => (StatusCode::CREATED, Json(tuning)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch tuning after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create tuning: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_tunings(Extension(state): Extension<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Tuning,
        "SELECT tuning_id, car_id, tuning_name, tuning_date as 'tuning_date: _', tuning_description, created_at, updated_at FROM Tunings"
    )
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
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Tuning,
        "SELECT tuning_id, car_id, tuning_name, tuning_date as 'tuning_date: _', tuning_description, created_at, updated_at FROM Tunings WHERE tuning_id = ?",
        id
    )
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
    Json(updated_tuning): Json<Tuning>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE Tunings SET car_id = ?, tuning_name = ?, tuning_date = ?, tuning_description = ? WHERE tuning_id = ?",
        updated_tuning.car_id,
        updated_tuning.tuning_name,
        updated_tuning.tuning_date,
        updated_tuning.tuning_description,
        id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                Tuning,
                "SELECT tuning_id, car_id, tuning_name, tuning_date, tuning_description, created_at, updated_at FROM Tunings WHERE tuning_id = ?",
                id
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(tuning) => (StatusCode::OK, Json(tuning)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch tuning after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update tuning: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_tuning(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM Tunings WHERE tuning_id = ?", id)
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
