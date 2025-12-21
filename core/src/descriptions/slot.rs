use serde::Deserialize;

/// Структура для слота транспортного средства
#[derive(Deserialize, Debug, Clone)]
pub struct Slot {
    pub id: String,
    pub slot_type: String,
    pub mount_point: String,
}
