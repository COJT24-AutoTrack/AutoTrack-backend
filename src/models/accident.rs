use serde::{Deserialize, Serialize};
use chrono::{NaiveDate,NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Accident {
    pub accident_id: i32,
    pub car_id: i32,
    pub accident_date: NaiveDate,
    pub accident_description: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
