use hecs::Entity;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AttachedItems(HashMap<String, Entity>);

impl AttachedItems {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn attach(&mut self, slot_id: &str, item: Entity) {
        self.0.insert(slot_id.to_string(), item);
    }

    pub fn detach(&mut self, slot_id: &str) {
        self.0.remove(slot_id);
    }

    pub fn get(&self, slot_id: &str) -> Option<Entity> {
        self.0.get(slot_id).copied()
    }
}

impl Serialize for AttachedItems {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
