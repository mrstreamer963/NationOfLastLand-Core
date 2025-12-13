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

pub fn add_guid_entity(guid: Guid, entity: Entity) {
    INTERNAL_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.guid_to_entity.insert(guid.clone(), entity);
        data.entity_to_guid.insert(entity, guid);
    });
}

pub fn get_entity_by_guid(guid: &Guid) -> Option<Entity> {
    INTERNAL_DATA.with(|data| {
        let data = data.borrow();
        data.guid_to_entity.get(guid).copied()
    })
}

pub fn get_guid_by_entity(entity: &Entity) -> Option<Guid> {
    INTERNAL_DATA.with(|data| {
        let data = data.borrow();
        data.entity_to_guid.get(entity).cloned()
    })
}

pub fn remove_by_guid(guid: &Guid) {
    INTERNAL_DATA.with(|data| {
        let mut data = data.borrow_mut();
        if let Some(entity) = data.guid_to_entity.remove(guid) {
            data.entity_to_guid.remove(&entity);
        }
    });
}

pub fn remove_by_entity(entity: &Entity) {
    INTERNAL_DATA.with(|data| {
        let mut data = data.borrow_mut();
        if let Some(guid) = data.entity_to_guid.remove(entity) {
            data.guid_to_entity.remove(&guid);
        }
    });
}
