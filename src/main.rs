mod db;
mod handlers;
mod models;
mod routes;
mod middleware;
mod state;

use axum;
use tokio::sync::Mutex;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use state::AppState;
use std::env;

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let env_filter = EnvFilter::from_default_env();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let db_pool = db::establish_connection().await;

    let state = Arc::new(Mutex::new(AppState {
        db_pool,
        firebase_project_id: env::var("FIREBASE_PROJECT_ID").expect("FIREBASE_PROJECT_ID must be set"),
        require_email_verification: env::var("REQUIRE_EMAIL_VERIFICATION")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .expect("REQUIRE_EMAIL_VERIFICATION must be a boolean"),
    }));

    let app = routes::create_routes(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8369));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
