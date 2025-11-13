use crate::modules::components::Pos;
use crate::modules::entities::Vehicle;
use crate::modules::entities::Waste;
use hecs::World;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct ExportData {
    wastes: Vec<HashMap<String, Pos>>,
    vehicles: Vec<HashMap<String, Pos>>,
}

pub fn export_to_json(world: &World) -> String {
    let mut wastes = Vec::new();
    let mut vehicles = Vec::new();

    // Выборка всех waste
    for (_id, (pos, _waste)) in world.query::<(&Pos, &Waste)>().iter() {
        wastes.push(HashMap::from([("pos".to_string(), *pos)]));
    }

    // Выборка всех vehicle
    for (_id, (pos, _vehicle)) in world.query::<(&Pos, &Vehicle)>().iter() {
        vehicles.push(HashMap::from([("pos".to_string(), *pos)]));
    }

    let data = ExportData { wastes, vehicles };
    serde_json::to_string(&data).unwrap()
}
