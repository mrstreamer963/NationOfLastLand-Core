use serde::Serialize;
use hecs::Entity;

#[derive(Serialize, Clone)]
pub struct Floors(pub Vec<Entity>);
