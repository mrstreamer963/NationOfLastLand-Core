use serde::Serialize;

/// Тип слота
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum SlotType {
    Main,
    Auxiliary,
}

impl SlotType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "main" => Ok(SlotType::Main),
            "auxiliary" => Ok(SlotType::Auxiliary),
            _ => Err(format!("Unknown slot type: {}", s)),
        }
    }
}

/// Структура для одного активного слота на транспортном средстве
#[derive(Clone, Debug, Serialize)]
pub struct ActiveSlot {
    pub id: String,
    pub slot_type: SlotType,
    pub mount_point: String,
}

/// Компонент для хранения активных слотов транспортного средства
#[derive(Clone, Debug, Serialize)]
pub struct ActiveSlots {
    pub slots: Vec<ActiveSlot>,
}

impl ActiveSlots {
    pub fn new(slots: Vec<ActiveSlot>) -> Self {
        Self { slots }
    }
}
