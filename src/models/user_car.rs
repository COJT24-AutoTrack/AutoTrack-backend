use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCar {
    pub firebase_user_id: String,
    pub car_id: i32,
}
