use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::state::AppState;
use crate::models::periodic_inspection::PeriodicInspection;

pub async fn create_periodic_inspection(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_periodic_inspection): Json<PeriodicInspection>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO PeriodicInspection (car_id, pi_name, pi_date, pi_nextdate) VALUES (?, ?, ?, ?)",
        new_periodic_inspection.car_id,
        new_periodic_inspection.pi_name,
        new_periodic_inspection.pi_date,
        new_periodic_inspection.pi_nextdate
    )
    .execute(&db_pool)
    .await
    {
        Ok(result) => {
            match query_as!(
                PeriodicInspection,
                "SELECT pi_id, car_id, pi_name, pi_date, pi_nextdate, created_at, updated_at FROM PeriodicInspection WHERE pi_id = ?",
                result.last_insert_id()
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(periodic_inspection) => (StatusCode::CREATED, Json(periodic_inspection)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch periodic inspection after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create periodic inspection: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


pub async fn get_periodic_inspections(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(PeriodicInspection, "SELECT * FROM PeriodicInspection")
        .fetch_all(&db_pool)
        .await
    {
        Ok(periodic_inspections) => (StatusCode::OK, Json(periodic_inspections)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch periodic inspections: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_periodic_inspection(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(pi_id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(PeriodicInspection, "SELECT * FROM PeriodicInspection WHERE pi_id = ?", pi_id)
        .fetch_one(&db_pool)
        .await
    {
        Ok(periodic_inspection) => (StatusCode::OK, Json(periodic_inspection)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch periodic inspection: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn update_periodic_inspection(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(pi_id): Path<i32>,
    Json(updated_periodic_inspection): Json<PeriodicInspection>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE PeriodicInspection SET car_id = ?, pi_name = ?, pi_date = ?, pi_nextdate = ? WHERE pi_id = ?",
        updated_periodic_inspection.car_id,
        updated_periodic_inspection.pi_name,
        updated_periodic_inspection.pi_date,
        updated_periodic_inspection.pi_nextdate,
        pi_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                PeriodicInspection,
                "SELECT pi_id, car_id, pi_name, pi_date, pi_nextdate, created_at, updated_at FROM PeriodicInspection WHERE pi_id = ?",
                pi_id
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(periodic_inspection) => (StatusCode::OK, Json(periodic_inspection)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch periodic inspection after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update periodic inspection: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


pub async fn delete_periodic_inspection(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(pi_id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM PeriodicInspection WHERE pi_id = ?", pi_id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Failed to delete periodic inspection: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
