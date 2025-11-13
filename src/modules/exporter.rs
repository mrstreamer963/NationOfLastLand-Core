use crate::modules::components::Pos;
use crate::modules::entities::Vehicle;
use crate::modules::entities::Waste;
use hecs::World;
use serde::Serialize;

#[derive(Serialize)]
struct ExportData {
    wastes: Vec<Pos>,
    vehicles: Vec<Pos>,
}

pub fn export_to_json(world: &World) -> String {
    let mut wastes = Vec::new();
    let mut vehicles = Vec::new();

    // Выборка всех waste
    for (_id, (pos, waste)) in world.query::<(&Pos, &Waste)>().iter() {
        wastes.push(*pos);
    }

    // Выборка всех vehicle
    for (_id, (pos, vehicle)) in world.query::<(&Pos, &Vehicle)>().iter() {
        vehicles.push(*pos);
    }

    let data = ExportData { wastes, vehicles };
    serde_json::to_string(&data).unwrap()
}
