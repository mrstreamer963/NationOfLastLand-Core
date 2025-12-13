use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AttachFloorEvent {
    pub base: hecs::Entity,
    pub floor: hecs::Entity,
}
