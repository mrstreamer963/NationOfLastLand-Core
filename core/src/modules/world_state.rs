use std::collections::HashMap;
use crate::modules::components::Guid;
use hecs::Entity;

#[derive(Debug, Default)]
pub struct WorldState {
    pub guid_to_entity: HashMap<Guid, Entity>,
}
