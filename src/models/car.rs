use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub car_id: i32,
    pub car_name: String,
    pub carmodelnum: String,
    pub car_color: String,
    pub car_mileage: i32,
    pub car_isflooding: bool,
    pub car_issmoked: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
