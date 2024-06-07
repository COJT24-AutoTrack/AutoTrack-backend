use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCar {
    pub user_id: i32,
    pub car_id: i32,
}
