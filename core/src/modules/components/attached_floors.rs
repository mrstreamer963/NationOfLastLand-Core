use hecs::Entity;
use serde::Serialize;

#[derive(Clone)]
pub struct AttachedFloors(pub Vec<Entity>);

impl AttachedFloors {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn attach(&mut self, floor: Entity) {
        self.0.push(floor);
    }
}

impl Serialize for AttachedFloors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
