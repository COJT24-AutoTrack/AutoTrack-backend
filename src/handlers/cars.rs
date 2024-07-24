use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query, MySql, Transaction};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::state::AppState;
use crate::models::car::Car;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCarRequest {
    car: Car,
    user_id: i32,
}

pub async fn create_car(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(req): Json<CreateCarRequest>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let CreateCarRequest { car, user_id } = req;

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
                "INSERT INTO user_car (user_id, car_id) VALUES (?, ?)",
                user_id,
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

pub async fn get_cars(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
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
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Car,
        "SELECT car_id, car_name, carmodelnum, car_color, car_mileage, car_isflooding as `car_isflooding: bool`, car_issmoked as `car_issmoked: bool`, car_image_url, created_at, updated_at FROM Cars WHERE car_id = ?",
        id
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
    Path(id): Path<i32>,
    Json(updated_car): Json<Car>
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
        id
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => {
            match query_as!(
                Car,
                "SELECT car_id, car_name, carmodelnum, car_color, car_mileage, car_isflooding as `car_isflooding: bool`, car_issmoked as `car_issmoked: bool`, car_image_url, created_at, updated_at FROM Cars WHERE car_id = ?",
                id
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
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let mut tx: Transaction<'_, MySql> = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let user_car_result = query!(
        "DELETE FROM user_car WHERE car_id = ?",
        id
    )
    .execute(&mut *tx)
    .await;

    match user_car_result {
        Ok(_) => {
            let car_result = query!(
                "DELETE FROM Cars WHERE car_id = ?",
                id
            )
            .execute(&mut *tx)
            .await;

            match car_result {
                Ok(_) => {
                    tx.commit().await.unwrap();
                    StatusCode::NO_CONTENT.into_response()
                }
                Err(e) => {
                    tx.rollback().await.unwrap();
                    eprintln!("Failed to delete car: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            tx.rollback().await.unwrap();
            eprintln!("Failed to delete user_car entry: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_car_image(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(car_id): Path<i32>,
    Json(image_url): Json<String>
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

pub async fn get_user_cars(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Car,
        "SELECT c.car_id, c.car_name, c.carmodelnum, c.car_color, c.car_mileage, c.car_isflooding as `car_isflooding: bool`, c.car_issmoked as `car_issmoked: bool`, c.car_image_url, c.created_at, c.updated_at 
         FROM Cars c
         JOIN user_car uc ON c.car_id = uc.car_id
         WHERE uc.user_id = ?",
        user_id
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