use serde::{Deserialize, Serialize};
use chrono::{NaiveDate,NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
    pub maint_id: i32,
    pub car_id: i32,
    pub maint_type: String,
    pub maint_date: NaiveDate,
    pub maint_description: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
