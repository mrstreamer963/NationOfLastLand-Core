use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum EntityType {
    Waste,
    Trash,
    Vehicle
}
