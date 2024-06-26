use axum::{Router, routing::get, routing::post, routing::put};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::AppState;

use crate::handlers::{
    users::{create_user, get_users, get_user, update_user, delete_user},
    cars::{create_car, get_cars, get_car, update_car, delete_car, update_car_image, get_user_cars},
    tunings::{create_tuning, get_tunings, get_tuning, update_tuning, delete_tuning},
    maintenances::{create_maintenance, get_maintenances, get_maintenance, update_maintenance, delete_maintenance},
    fuel_efficiencies::{create_fuel_efficiency, get_fuel_efficiencies, get_fuel_efficiency, update_fuel_efficiency, delete_fuel_efficiency},
    accidents::{create_accident, get_accidents, get_accident, update_accident, delete_accident},
    periodic_inspections::{create_periodic_inspection, get_periodic_inspections, get_periodic_inspection, update_periodic_inspection, delete_periodic_inspection},
    images::upload_image,
};

pub fn create_routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }).post(|| async { "Hello, world!" }))
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/users/:user_id/cars", get(get_user_cars))
        .route("/cars", post(create_car).get(get_cars))
        .route("/cars/:id", get(get_car).put(update_car).delete(delete_car))
        .route("/cars/:id/image", put(update_car_image))
        .route("/tunings", post(create_tuning).get(get_tunings))
        .route("/tunings/:id", get(get_tuning).put(update_tuning).delete(delete_tuning))
        .route("/maintenances", post(create_maintenance).get(get_maintenances))
        .route("/maintenances/:id", get(get_maintenance).put(update_maintenance).delete(delete_maintenance))
        .route("/fuel_efficiencies", post(create_fuel_efficiency).get(get_fuel_efficiencies))
        .route("/fuel_efficiencies/:id", get(get_fuel_efficiency).put(update_fuel_efficiency).delete(delete_fuel_efficiency))
        .route("/accidents", post(create_accident).get(get_accidents))
        .route("/accidents/:id", get(get_accident).put(update_accident).delete(delete_accident))
        .route("/periodic_inspections", post(create_periodic_inspection).get(get_periodic_inspections))
        .route("/periodic_inspections/:id", get(get_periodic_inspection).put(update_periodic_inspection).delete(delete_periodic_inspection))
        .route("/upload_image", post(upload_image))
        .layer(axum::Extension(state))
}
