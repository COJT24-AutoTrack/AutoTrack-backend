use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::user::User;

pub async fn create_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        r#"
        INSERT INTO Users (user_email, user_name, user_password)
        VALUES ($1, $2, $3)
        RETURNING user_id, user_email, user_name, user_password, created_at, updated_at
        "#,
        new_user.user_email,
        new_user.user_name,
        new_user.user_password
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
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
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(User, "SELECT * FROM Users WHERE user_id = $1", id)
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
    Path(id): Path<i32>,
    Json(updated_user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        r#"
        UPDATE Users
        SET user_email = $1, user_name = $2, user_password = $3, updated_at = CURRENT_TIMESTAMP
        WHERE user_id = $4
        RETURNING user_id, user_email, user_name, user_password, created_at, updated_at
        "#,
        updated_user.user_email,
        updated_user.user_name,
        updated_user.user_password,
        id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => {
            eprintln!("Failed to update user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!("DELETE FROM Users WHERE user_id = $1", id)
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
