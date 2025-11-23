use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum UnitType {
    Vehicle,
    Waste,
    Trash
}
