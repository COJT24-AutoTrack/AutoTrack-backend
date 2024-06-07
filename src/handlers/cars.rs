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
            let car_result = query!(
                r#"
                INSERT INTO Cars (car_name, carmodelnum, car_color, car_milage, car_isflooding, car_issmoked)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                car.car_name,
                car.carmodelnum,
                car.car_color,
                car.car_milage,
                car.car_isflooding,
                car.car_issmoked
            )
            .execute(&mut tx)
            .await;

            match car_result {
                Ok(res) => {
                    let car_id = res.last_insert_id();
                    let car = query_as!(Car, "SELECT * FROM Cars WHERE car_id = ?", car_id)
                        .fetch_one(&mut tx)
                        .await
                        .unwrap();

                    // user_carテーブルにエントリを追加
                    let user_car_result = query!(
                        r#"
                        INSERT INTO user_car (user_id, car_id)
                        VALUES (?, ?)
                        "#,
                        user_id,
                        car_id
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

    match query_as!(Car, "SELECT * FROM Cars WHERE car_id = ?", id)
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
        r#"
        UPDATE Cars
        SET car_name = ?, carmodelnum = ?, car_color = ?, car_milage = ?, car_isflooding = ?, car_issmoked = ?, updated_at = CURRENT_TIMESTAMP
        WHERE car_id = ?
        "#,
        updated_car.car_name,
        updated_car.carmodelnum,
        updated_car.car_color,
        updated_car.car_milage,
        updated_car.car_isflooding,
        updated_car.car_issmoked,
        id
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => {
            match query_as!(Car, "SELECT * FROM Cars WHERE car_id = ?", id)
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

    let result = db_pool.begin().await;
    match result {
        Ok(mut tx) => {
            // user_carテーブルからエントリを削除
            let user_car_result = query!(
                r#"
                DELETE FROM user_car WHERE car_id = ?
                "#,
                id
            )
            .execute(&mut tx)
            .await;

            match user_car_result {
                Ok(_) => {
                    // Carsテーブルからエントリを削除
                    let car_result = query!(
                        "DELETE FROM Cars WHERE car_id = ?",
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
