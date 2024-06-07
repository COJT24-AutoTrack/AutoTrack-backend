use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::maintenance::Maintenance;

pub async fn create_maintenance(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_maintenance): Json<Maintenance>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Maintenance,
        r#"
        INSERT INTO Maintenances (car_id, maint_type, maint_date, maint_description)
        VALUES ($1, $2, $3, $4)
        RETURNING maint_id, car_id, maint_type, maint_date, maint_description, created_at, updated_at
        "#,
        new_maintenance.car_id,
        new_maintenance.maint_type,
        new_maintenance.maint_date,
        new_maintenance.maint_description
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(maintenance) => (StatusCode::CREATED, Json(maintenance)).into_response(),
        Err(e) => {
            eprintln!("Failed to create maintenance: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_maintenances(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Maintenance, "SELECT * FROM Maintenances")
        .fetch_all(&db_pool)
        .await
    {
        Ok(maintenances) => (StatusCode::OK, Json(maintenances)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch maintenances: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_maintenance(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Maintenance, "SELECT * FROM Maintenances WHERE maint_id = $1", id)
        .fetch_one(&db_pool)
        .await
    {
        Ok(maintenance) => (StatusCode::OK, Json(maintenance)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch maintenance: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn update_maintenance(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
    Json(updated_maintenance): Json<Maintenance>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Maintenance,
        r#"
        UPDATE Maintenances
        SET car_id = $1, maint_type = $2, maint_date = $3, maint_description = $4, updated_at = CURRENT_TIMESTAMP
        WHERE maint_id = $5
        RETURNING maint_id, car_id, maint_type, maint_date, maint_description, created_at, updated_at
        "#,
        updated_maintenance.car_id,
        updated_maintenance.maint_type,
        updated_maintenance.maint_date,
        updated_maintenance.maint_description,
        id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(maintenance) => (StatusCode::OK, Json(maintenance)).into_response(),
        Err(e) => {
            eprintln!("Failed to update maintenance: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_maintenance(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM Maintenances WHERE maint_id = $1", id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Failed to delete maintenance: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
