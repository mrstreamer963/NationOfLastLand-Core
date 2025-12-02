use hecs::Entity;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AttachedItems {
    pub items: HashMap<String, Entity>,
}

impl AttachedItems {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn attach(&mut self, slot_id: &str, item: Entity) {
        self.items.insert(slot_id.to_string(), item);
    }

    pub fn detach(&mut self, slot_id: &str) {
        self.items.remove(slot_id);
    }

    pub fn get(&self, slot_id: &str) -> Option<Entity> {
        self.items.get(slot_id).copied()
    }
}

impl Serialize for AttachedItems {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.items.serialize(serializer)
    }
}
