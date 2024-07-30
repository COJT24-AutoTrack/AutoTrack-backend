use crate::models::maintenance::{Maintenance, MAINTENANCE_TITLES};
use crate::state::AppState;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{query, query_as};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn apply_maintenance_logic(mut maintenance: Maintenance) -> Maintenance {
    if maintenance.maint_type == "Other" {
        // maint_titleはそのまま
    } else if let Some(&title) = MAINTENANCE_TITLES.get(maintenance.maint_type.as_str()) {
        maintenance.maint_title = title.to_string();
    }
    maintenance
}

pub async fn create_maintenance(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_maintenance): Json<Maintenance>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let new_maintenance = apply_maintenance_logic(new_maintenance);

    match query!(
        "INSERT INTO Maintenances (car_id, maint_type, maint_title, maint_date, maint_description) VALUES (?, ?, ?, ?, ?)",
        new_maintenance.car_id,
        new_maintenance.maint_type,
        new_maintenance.maint_title,
        new_maintenance.maint_date,
        new_maintenance.maint_description
    )
    .execute(&db_pool)
    .await
    {
        Ok(result) => {
            match query_as!(
                Maintenance,
                "SELECT * FROM Maintenances WHERE maint_id = ?",
                result.last_insert_id()
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(maintenance) => (StatusCode::CREATED, Json(maintenance)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch maintenance after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create maintenance: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_maintenances(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
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
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Maintenance,
        "SELECT * FROM Maintenances WHERE maint_id = ?",
        id
    )
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
    Json(updated_maintenance): Json<Maintenance>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE Maintenances SET car_id = ?, maint_type = ?, maint_title = ?, maint_date = ?, maint_description = ? WHERE maint_id = ?",
        updated_maintenance.car_id,
        updated_maintenance.maint_type,
        updated_maintenance.maint_title,
        updated_maintenance.maint_date,
        updated_maintenance.maint_description,
        id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                Maintenance,
                "SELECT maint_id, car_id, maint_type, maint_date, maint_description, created_at, updated_at FROM Maintenances WHERE maint_id = ?",
                id
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(maintenance) => (StatusCode::OK, Json(maintenance)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch maintenance after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update maintenance: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_maintenance(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM Maintenances WHERE maint_id = ?", id)
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
