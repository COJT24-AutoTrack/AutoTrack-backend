use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub car_id: i32,
    pub car_name: String,
    pub carmodelnum: String,
    pub car_color: String,
    pub car_milage: i32,
    pub car_isflooding: bool,
    pub car_issmoked: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
