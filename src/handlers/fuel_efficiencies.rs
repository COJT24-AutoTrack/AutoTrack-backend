use crate::models::fuel_efficiency::FuelEfficiency;
use crate::state::AppState;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::{query, query_as};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_fuel_efficiency(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_fuel_efficiency): Json<FuelEfficiency>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO FuelEfficiencies (car_id, fe_date, fe_amount, fe_unitprice, fe_mileage) VALUES (?, ?, ?, ?, ?)",
        new_fuel_efficiency.car_id,
        new_fuel_efficiency.fe_date,
        new_fuel_efficiency.fe_amount,
        new_fuel_efficiency.fe_unitprice,
        new_fuel_efficiency.fe_mileage
    )
    .execute(&db_pool)
    .await
    {
        Ok(result) => {
            match query_as!(
                FuelEfficiency,
                "SELECT fe_id, car_id, fe_date, fe_amount, fe_unitprice, fe_mileage, created_at, updated_at FROM FuelEfficiencies WHERE fe_id = ?",
                result.last_insert_id()
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(fuel_efficiency) => (StatusCode::CREATED, Json(fuel_efficiency)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch fuel efficiency after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create fuel efficiency: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_fuel_efficiencies(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
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
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        FuelEfficiency,
        "SELECT * FROM FuelEfficiencies WHERE fe_id = ?",
        id
    )
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
    Json(updated_fuel_efficiency): Json<FuelEfficiency>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE FuelEfficiencies SET car_id = ?, fe_date = ?, fe_amount = ?, fe_unitprice = ?, fe_mileage = ? WHERE fe_id = ?",
        updated_fuel_efficiency.car_id,
        updated_fuel_efficiency.fe_date,
        updated_fuel_efficiency.fe_amount,
        updated_fuel_efficiency.fe_unitprice,
        updated_fuel_efficiency.fe_mileage,
        id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                FuelEfficiency,
                "SELECT fe_id, car_id, fe_date, fe_amount, fe_unitprice, fe_mileage, created_at, updated_at FROM FuelEfficiencies WHERE fe_id = ?",
                id
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(fuel_efficiency) => (StatusCode::OK, Json(fuel_efficiency)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch fuel efficiency after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update fuel efficiency: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_fuel_efficiency(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM FuelEfficiencies WHERE fe_id = ?", id)
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

pub async fn calculate_fuel_efficiencies(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    // 給油記録を取得
    let fuel_efficiencies: Vec<FuelEfficiency> = match query_as!(
        FuelEfficiency,
        "SELECT fe_id, car_id, fe_date, fe_amount, fe_unitprice, fe_mileage, created_at, updated_at 
         FROM FuelEfficiencies 
         WHERE car_id = ? 
         ORDER BY fe_date",
        car_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Failed to fetch fuel efficiencies: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if fuel_efficiencies.len() < 2 {
        return (
            StatusCode::BAD_REQUEST,
            "Not enough data to calculate fuel efficiency",
        )
            .into_response();
    }

    let mut total_fuel = 0.0;
    let mut total_distance = 0;
    let mut fuel_efficiency_records = Vec::new();

    for i in 1..fuel_efficiencies.len() {
        let current = &fuel_efficiencies[i];

        let distance = current.fe_mileage;
        let fuel = current.fe_amount;

        if distance <= 0 || fuel <= 0.0 {
            return (StatusCode::BAD_REQUEST, "Invalid mileage or fuel data").into_response();
        }

        let efficiency = distance as f32 / fuel;
        fuel_efficiency_records.push(json!({
            "fe_id": current.fe_id,
            "fuel_efficiency": efficiency
        }));

        total_fuel += fuel;
        total_distance += distance;
    }

    if total_distance == 0 {
        return (
            StatusCode::BAD_REQUEST,
            "Total distance is zero, cannot calculate fuel efficiency",
        )
            .into_response();
    }

    let total_fuel_efficiency = total_distance as f32 / total_fuel;
    let response = json!({
        "car_id": car_id,
        "total_fuel_efficiency": total_fuel_efficiency,
        "fuel_efficiencies": fuel_efficiency_records
    });

    (StatusCode::OK, Json(response)).into_response()
}
