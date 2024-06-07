use serde::{Deserialize, Serialize};
use chrono::{NaiveDate,NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tuning {
    pub tuning_id: i32,
    pub car_id: i32,
    pub tuning_name: String,
    pub tuning_date: NaiveDate,
    pub tuning_description: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
