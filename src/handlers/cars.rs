use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::car::Car;

pub async fn create_car(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_car): Json<(Car, i32)>  // Carとuser_idのタプルを受け取る
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let (car, user_id) = new_car;

    let result = db_pool.begin().await;
    match result {
        Ok(mut tx) => {
            // 車を作成
            let car_result = query_as!(
                Car,
                r#"
                INSERT INTO Cars (car_name, carmodelnum, car_color, car_milage, car_isflooding, car_issmoked)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING car_id, car_name, carmodelnum, car_color, car_milage, car_isflooding, car_issmoked, created_at, updated_at
                "#,
                car.car_name,
                car.carmodelnum,
                car.car_color,
                car.car_milage,
                car.car_isflooding,
                car.car_issmoked
            )
            .fetch_one(&mut tx)
            .await;

            match car_result {
                Ok(car) => {
                    // user_carテーブルにエントリを追加
                    let user_car_result = query!(
                        r#"
                        INSERT INTO user_car (user_id, car_id)
                        VALUES ($1, $2)
                        "#,
                        user_id,
                        car.car_id
                    )
                    .execute(&mut tx)
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
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_cars(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(Car, "SELECT * FROM Cars")
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

    match query_as!(Car, "SELECT * FROM Cars WHERE car_id = $1", id)
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

    match query_as!(
        Car,
        r#"
        UPDATE Cars
        SET car_name = $1, carmodelnum = $2, car_color = $3, car_milage = $4, car_isflooding = $5, car_issmoked = $6, updated_at = CURRENT_TIMESTAMP
        WHERE car_id = $7
        RETURNING car_id, car_name, carmodelnum, car_color, car_milage, car_isflooding, car_issmoked, created_at, updated_at
        "#,
        updated_car.car_name,
        updated_car.carmodelnum,
        updated_car.car_color,
        updated_car.car_milage,
        updated_car.car_isflooding,
        updated_car.car_issmoked,
        id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(car) => (StatusCode::OK, Json(car)).into_response(),
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

    let result = db_pool.begin().await;
    match result {
        Ok(mut tx) => {
            // user_carテーブルからエントリを削除
            let user_car_result = query!(
                r#"
                DELETE FROM user_car WHERE car_id = $1
                "#,
                id
            )
            .execute(&mut tx)
            .await;

            match user_car_result {
                Ok(_) => {
                    // Carsテーブルからエントリを削除
                    let car_result = query!(
                        "DELETE FROM Cars WHERE car_id = $1",
                        id
                    )
                    .execute(&mut tx)
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
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
