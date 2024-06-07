use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::fuel_efficiency::FuelEfficiency;

pub async fn create_fuel_efficiency(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_fuel_efficiency): Json<FuelEfficiency>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        FuelEfficiency,
        r#"
        INSERT INTO FuelEfficiencies (car_id, fe_date, fe_amount, fe_unitprice, fe_milage)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING fe_id, car_id, fe_date, fe_amount, fe_unitprice, fe_milage, created_at, updated_at
        "#,
        new_fuel_efficiency.car_id,
        new_fuel_efficiency.fe_date,
        new_fuel_efficiency.fe_amount,
        new_fuel_efficiency.fe_unitprice,
        new_fuel_efficiency.fe_milage
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(fuel_efficiency) => (StatusCode::CREATED, Json(fuel_efficiency)).into_response(),
        Err(e) => {
            eprintln!("Failed to create fuel efficiency: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_fuel_efficiencies(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(FuelEfficiency, "SELECT * FROM FuelEfficiencies")
        .fetch_all(&db_pool)
        .await
    {
        Ok(fuel_efficiencies) => (StatusCode::OK, Json(fuel_efficiencies)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch fuel efficiencies: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_fuel_efficiency(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(FuelEfficiency, "SELECT * FROM FuelEfficiencies WHERE fe_id = $1", id)
        .fetch_one(&db_pool)
        .await
    {
        Ok(fuel_efficiency) => (StatusCode::OK, Json(fuel_efficiency)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch fuel efficiency: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn update_fuel_efficiency(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
    Json(updated_fuel_efficiency): Json<FuelEfficiency>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        FuelEfficiency,
        r#"
        UPDATE FuelEfficiencies
        SET car_id = $1, fe_date = $2, fe_amount = $3, fe_unitprice = $4, fe_milage = $5, updated_at = CURRENT_TIMESTAMP
        WHERE fe_id = $6
        RETURNING fe_id, car_id, fe_date, fe_amount, fe_unitprice, fe_milage, created_at, updated_at
        "#,
        updated_fuel_efficiency.car_id,
        updated_fuel_efficiency.fe_date,
        updated_fuel_efficiency.fe_amount,
        updated_fuel_efficiency.fe_unitprice,
        updated_fuel_efficiency.fe_milage,
        id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(fuel_efficiency) => (StatusCode::OK, Json(fuel_efficiency)).into_response(),
        Err(e) => {
            eprintln!("Failed to update fuel efficiency: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_fuel_efficiency(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM FuelEfficiencies WHERE fe_id = $1", id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Failed to delete fuel efficiency: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
