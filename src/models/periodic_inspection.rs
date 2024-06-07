use serde::{Deserialize, Serialize};
use chrono::{NaiveDate,NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodicInspection {
    pub pi_id: i32,
    pub car_id: i32,
    pub pi_name: String,
    pub pi_date: NaiveDate,
    pub pi_nextdate: NaiveDate,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
