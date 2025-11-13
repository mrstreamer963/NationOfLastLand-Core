use crate::modules::components::Pos;
use crate::modules::entities::Vehicle;
use crate::modules::entities::Waste;
use hecs::World;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
struct ExportData {
    wastes: Vec<HashMap<String, Value>>,
    vehicles: Vec<HashMap<String, Value>>,
}

pub fn export_to_json(world: &World) -> String {
    let mut wastes = Vec::new();
    let mut vehicles = Vec::new();

    // Выборка всех waste
    for (_id, (pos, _waste)) in world.query::<(&Pos, &Waste)>().iter() {
        wastes.push(HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
        ]));
    }

    // Выборка всех vehicle
    for (_id, (pos, _vehicle)) in world.query::<(&Pos, &Vehicle)>().iter() {
        vehicles.push(HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
        ]));
    }

    let data = ExportData { wastes, vehicles };
    serde_json::to_string(&data).unwrap()
}
