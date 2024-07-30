use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
    pub maint_id: Option<i32>,
    pub car_id: i32,
    pub maint_type: String, // You can use an enum here if you want to strictly type the maintenance types
    pub maint_title: String,
    pub maint_date: OffsetDateTime,
    pub maint_description: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

lazy_static::lazy_static! {
    pub static ref MAINTENANCE_TITLES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Oil Change", "オイル交換");
        m.insert("Oil Filter Change", "オイルエレメント交換");
        m.insert("Headlight Change", "ヘッドライト交換");
        m.insert("Position Light Change", "ポジションライト交換");
        m.insert("Fog Light Change", "フォグライト交換");
        m.insert("Turn Signal Change", "ウインカー交換");
        m.insert("Brake Light Change", "ブレーキライト交換");
        m.insert("License Plate Light Change", "ナンバー灯交換");
        m.insert("Backup Light Change", "バックライト交換");
        m.insert("Car Wash", "洗車");
        m.insert("Wiper Blade Change", "ワイパーブレード交換");
        m.insert("Brake Pad Change", "ブレーキパッド交換");
        m.insert("Brake Disc Change", "ブレーキディスク交換");
        m.insert("Tire Change", "タイヤ交換");
        m.insert("Battery Change", "バッテリー交換");
        m.insert("Timing Belt Change", "タイミングベルト交換");
        m.insert("Coolant Refill", "クーラント補充");
        m.insert("Washer Fluid Refill", "ウォッシャー液補充");
        m
    };
}
