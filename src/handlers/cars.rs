use crate::models::car::Car;
use crate::models::fuel_efficiency::FuelEfficiency;
use crate::models::maintenance::Maintenance;
use crate::models::tuning::Tuning;
use crate::state::AppState;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, MySql, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCarRequest {
    car: Car,
    firebase_user_id: String,
}

pub async fn create_car(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(req): Json<CreateCarRequest>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let CreateCarRequest {
        car,
        firebase_user_id,
    } = req;

    let mut tx: Transaction<'_, MySql> = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let car_result = query!(
        "INSERT INTO Cars (car_name, carmodelnum, car_color, car_mileage, car_isflooding, car_issmoked, car_image_url) VALUES (?, ?, ?, ?, ?, ?, ?)",
        car.car_name,
        car.carmodelnum,
        car.car_color,
        car.car_mileage,
        car.car_isflooding as i8,
        car.car_issmoked as i8,
        car.car_image_url
    )
    .execute(&mut *tx)
    .await;

    match car_result {
        Ok(res) => {
            let car_id = res.last_insert_id();
            let car = query_as!(
                Car,
                "SELECT car_id, car_name, carmodelnum, car_color, car_mileage, car_isflooding as `car_isflooding: bool`, car_issmoked as `car_issmoked: bool`, car_image_url, created_at, updated_at FROM Cars WHERE car_id = ?",
                car_id
            )
            .fetch_one(&mut *tx)
            .await
            .unwrap();

            let user_car_result = query!(
                "INSERT INTO user_car (firebase_user_id, car_id) VALUES (?, ?)",
                firebase_user_id,
                car_id
            )
            .execute(&mut *tx)
            .await;

            match user_car_result {
                Ok(_) => {
                    tx.commit().await.unwrap();
                    (StatusCode::CREATED, Json(car)).into_response()
                }
                Err(e) => {
                    tx.rollback().await.unwrap();
                    eprintln!("Failed to create user_car entry: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            tx.rollback().await.unwrap();
            eprintln!("Failed to create car: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_cars(Extension(state): Extension<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Car,
        "SELECT car_id, car_name, carmodelnum, car_color, car_mileage, car_isflooding as `car_isflooding: bool`, car_issmoked as `car_issmoked: bool`, car_image_url, created_at, updated_at FROM Cars"
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(cars) => (StatusCode::OK, Json(cars)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch cars: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_car(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Car,
        "SELECT car_id, car_name, carmodelnum, car_color, car_mileage, car_isflooding as `car_isflooding: bool`, car_issmoked as `car_issmoked: bool`, car_image_url, created_at, updated_at FROM Cars WHERE car_id = ?",
        car_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(car) => (StatusCode::OK, Json(car)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch car: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn update_car(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
    Json(updated_car): Json<Car>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let result = query!(
        "UPDATE Cars SET car_name = ?, carmodelnum = ?, car_color = ?, car_mileage = ?, car_isflooding = ?, car_issmoked = ?, car_image_url = ? WHERE car_id = ?",
        updated_car.car_name,
        updated_car.carmodelnum,
        updated_car.car_color,
        updated_car.car_mileage,
        updated_car.car_isflooding as i8,
        updated_car.car_issmoked as i8,
        updated_car.car_image_url,
        car_id
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => {
            match query_as!(
                Car,
                "SELECT car_id, car_name, carmodelnum, car_color, car_mileage, car_isflooding as `car_isflooding: bool`, car_issmoked as `car_issmoked: bool`, car_image_url, created_at, updated_at FROM Cars WHERE car_id = ?",
                car_id
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(car) => (StatusCode::OK, Json(car)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch updated car: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to update car: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_car(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let mut tx: Transaction<'_, MySql> = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Delete related data from other tables
    let tables = vec![
        "FuelEfficiencies",
        "Maintenances",
        "Tunings",
        "Accidents",
        "PeriodicInspection",
        "user_car",
    ];

    for table in tables {
        let delete_result = query(&format!("DELETE FROM {} WHERE car_id = ?", table))
            .bind(car_id)
            .execute(&mut *tx)
            .await;

        if let Err(e) = delete_result {
            tx.rollback().await.unwrap();
            eprintln!("Failed to delete from {}: {:?}", table, e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    // Finally, delete the car itself
    let car_result = query!("DELETE FROM Cars WHERE car_id = ?", car_id)
        .execute(&mut *tx)
        .await;

    match car_result {
        Ok(_) => match tx.commit().await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => {
                eprintln!("Failed to commit transaction: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
        Err(e) => {
            tx.rollback().await.unwrap();
            eprintln!("Failed to delete car: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_car_image(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
    Json(image_url): Json<String>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let result = query!(
        "UPDATE Cars SET car_image_url = ? WHERE car_id = ?",
        image_url,
        car_id
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, "Image URL updated successfully").into_response(),
        Err(e) => {
            eprintln!("Failed to update car image URL: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
pub async fn delete_car_image(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let result = query!(
        "UPDATE Cars SET car_image_url = NULL WHERE car_id = ?",
        car_id
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, "Car image URL deleted successfully").into_response(),
        Err(e) => {
            eprintln!("Failed to delete car image URL: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_user_cars(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(firebase_user_id): Path<String>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Car,
        "SELECT c.car_id, c.car_name, c.carmodelnum, c.car_color, c.car_mileage, c.car_isflooding as `car_isflooding: bool`, c.car_issmoked as `car_issmoked: bool`, c.car_image_url, c.created_at, c.updated_at 
         FROM Cars c
         JOIN user_car uc ON c.car_id = uc.car_id
         WHERE uc.firebase_user_id = ?",
        firebase_user_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(cars) => (StatusCode::OK, Json(cars)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch user cars: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_car_tuning(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Tuning, "SELECT * FROM Tunings WHERE car_id = ?", car_id)
        .fetch_all(&db_pool)
        .await
    {
        Ok(tunings) => (StatusCode::OK, Json(tunings)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch car tunings: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_car_maintenance(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Maintenance,
        "SELECT * FROM Maintenances WHERE car_id = ?",
        car_id
    )
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

pub async fn get_car_fuel_efficiency(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        FuelEfficiency,
        "SELECT * FROM FuelEfficiencies WHERE car_id = ?",
        car_id
    )
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
