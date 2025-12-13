use hecs::Entity;

use crate::modules::components::Guid;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Owner{
    pub e: Entity,
    pub guid: Guid
}
