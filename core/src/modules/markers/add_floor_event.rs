use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AddFloorEvent {
    pub base: hecs::Entity,
    pub floor_type: String,
}
