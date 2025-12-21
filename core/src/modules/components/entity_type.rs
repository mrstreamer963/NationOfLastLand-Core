use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum EntityType {
    Waste,
    Trash,
    Item,
    Base,
    Floor,
    Unit            //  некий юнит c активными слотами под предметы
}
