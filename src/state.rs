// src/state.rs
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
    pub firebase_project_id: String,
    pub require_email_verification: bool,
}