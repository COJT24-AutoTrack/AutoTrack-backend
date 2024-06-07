use serde::{Deserialize, Serialize};
use chrono::{NaiveDate,NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelEfficiency {
    pub fe_id: i32,
    pub car_id: i32,
    pub fe_date: NaiveDate,
    pub fe_amount: f32,
    pub fe_unitprice: f32,
    pub fe_milage: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
