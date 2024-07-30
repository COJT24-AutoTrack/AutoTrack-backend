use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::state::AppState;
use crate::models::user::User;

pub async fn create_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO Users (firebase_user_id, user_email, user_name) VALUES (?, ?, ?)",
        new_user.firebase_user_id,
        new_user.user_email,
        new_user.user_name,
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                User,
                "SELECT * FROM Users WHERE user_email = ?",
                new_user.user_email
            )
            .fetch_one(&db_pool)
            .await {
                Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch user after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_users(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(User, "SELECT * FROM Users")
        .fetch_all(&db_pool)
        .await
    {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch users: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(firebase_user_id): Path<String>  // firebase_user_id は String 型
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(User, "SELECT * FROM Users WHERE firebase_user_id = ?", firebase_user_id)
        .fetch_one(&db_pool)
        .await
    {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch user: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn update_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(firebase_user_id): Path<String>,  // firebase_user_id は String 型
    Json(updated_user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE Users SET user_email = ?, user_name = ?, firebase_user_id = ? WHERE firebase_user_id = ?",
        updated_user.user_email,
        updated_user.user_name,
        updated_user.firebase_user_id,
        firebase_user_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(User, "SELECT * FROM Users WHERE firebase_user_id = ?", firebase_user_id)
            .fetch_one(&db_pool)
            .await {
                Ok(user) => (StatusCode::OK, Json(user)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch user after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(firebase_user_id): Path<String>  // firebase_user_id は String 型
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM Users WHERE firebase_user_id = ?", firebase_user_id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Failed to delete user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
