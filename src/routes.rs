use crate::state::AppState;
use axum::http::{header, Method};
use axum::{
    extract::DefaultBodyLimit, middleware::from_fn_with_state, routing::get, routing::post,
    routing::put, Extension, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use crate::handlers::{
    accidents, cars, fuel_efficiencies, images, maintenances, periodic_inspections, tunings, users,
};
use crate::middleware::auth::jwt_auth;

use axum::Json;
use serde_json::json;

pub async fn test() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello SSL!" }))
}

pub fn create_routes(state: Arc<Mutex<AppState>>) -> Router {
    let user_routes = Router::new()
        .route("/", post(users::create_user).get(users::get_users))
        .route(
            "/:user_id",
            get(users::get_user)
                .put(users::update_user)
                .delete(users::delete_user),
        )
        .route("/:user_id/cars", get(cars::get_user_cars));

    let car_routes = Router::new()
        .route("/", post(cars::create_car).get(cars::get_cars))
        .route(
            "/:car_id",
            get(cars::get_car)
                .put(cars::update_car)
                .delete(cars::delete_car),
        )
        .route(
            "/:car_id/image",
            put(cars::update_car_image).delete(cars::delete_car_image),
        )
        .route("/:car_id/tuning", get(cars::get_car_tuning))
        .route("/:car_id/maintenance", get(cars::get_car_maintenance))
        .route(
            "/:car_id/fuel_efficiency",
            get(cars::get_car_fuel_efficiency),
        );

    let tuning_routes = Router::new()
        .route("/", post(tunings::create_tuning).get(tunings::get_tunings))
        .route(
            "/:tuning_id",
            get(tunings::get_tuning)
                .put(tunings::update_tuning)
                .delete(tunings::delete_tuning),
        );

    let maintenance_routes = Router::new()
        .route(
            "/",
            post(maintenances::create_maintenance).get(maintenances::get_maintenances),
        )
        .route(
            "/:maint_id",
            get(maintenances::get_maintenance)
                .put(maintenances::update_maintenance)
                .delete(maintenances::delete_maintenance),
        );

    let fuel_efficiency_routes = Router::new()
        .route(
            "/",
            post(fuel_efficiencies::create_fuel_efficiency)
                .get(fuel_efficiencies::get_fuel_efficiencies),
        )
        .route(
            "/:fe_id",
            get(fuel_efficiencies::get_fuel_efficiency)
                .put(fuel_efficiencies::update_fuel_efficiency)
                .delete(fuel_efficiencies::delete_fuel_efficiency),
        )
        .route(
            "/:car_id/fuel_efficiencies/calculate",
            get(fuel_efficiencies::calculate_fuel_efficiencies),
        );

    let accident_routes = Router::new()
        .route(
            "/",
            post(accidents::create_accident).get(accidents::get_accidents),
        )
        .route(
            "/:accident_id",
            get(accidents::get_accident)
                .put(accidents::update_accident)
                .delete(accidents::delete_accident),
        );

    let periodic_inspection_routes = Router::new()
        .route(
            "/",
            post(periodic_inspections::create_periodic_inspection)
                .get(periodic_inspections::get_periodic_inspections),
        )
        .route(
            "/:pi_id",
            get(periodic_inspections::get_periodic_inspection)
                .put(periodic_inspections::update_periodic_inspection)
                .delete(periodic_inspections::delete_periodic_inspection),
        );

    let test_routes = Router::new().route("/", get(test));

    let image_routes = Router::new().route("/", post(images::upload_image));

    let public_routes = Router::new().route("/test", get(test));

    let private_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/cars", car_routes)
        .nest("/tunings", tuning_routes)
        .nest("/maintenances", maintenance_routes)
        .nest("/fuel_efficiencies", fuel_efficiency_routes)
        .nest("/accidents", accident_routes)
        .nest("/periodic_inspections", periodic_inspection_routes)
        .nest("/images", image_routes)
        .nest("/test", test_routes)
        .layer(from_fn_with_state(Arc::clone(&state), jwt_auth));

    Router::new()
        .merge(public_routes)
        .nest("/api", private_routes)
        .with_state(state.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                .expose_headers([header::CONTENT_DISPOSITION])
                .allow_methods([Method::GET, Method::PUT, Method::POST, Method::DELETE])
                .allow_origin(Any),
        )
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100MBのボディサイズ制限
        .layer(Extension(state))
}
struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let Some(schema) = openapi.components.as_mut() else {
            return;
        };
        schema.add_security_scheme(
            "jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
