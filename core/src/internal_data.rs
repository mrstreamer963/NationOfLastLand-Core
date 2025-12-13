use std::cell::RefCell;
use std::collections::HashMap;
use hecs::Entity;
use crate::modules::components::Guid;

#[derive(Default)]
pub struct InternalData {
    pub guid_to_entity: HashMap<Guid, Entity>,
    pub entity_to_guid: HashMap<Entity, Guid>,
}

thread_local! {
    pub static INTERNAL_DATA: RefCell<InternalData> = RefCell::new(InternalData::default());
}
