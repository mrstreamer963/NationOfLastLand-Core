use hecs::Entity;
use serde::Serialize;

#[derive(Clone)]
pub struct Inventory(pub Vec<Entity>);

impl Inventory {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, item: Entity) {
        self.0.push(item);
    }

    pub fn remove(&mut self, index: usize) -> Option<Entity> {
        if index < self.0.len() {
            Some(self.0.remove(index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<Entity> {
        self.0.get(index).copied()
    }
}

impl Serialize for Inventory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
