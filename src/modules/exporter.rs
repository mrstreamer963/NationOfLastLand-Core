use hecs::World;
use serde::Serialize;
use crate::modules::components::Pos;
use crate::modules::entities::Vehicle;
use crate::modules::entities::Waste;

#[derive(Serialize)]
struct ExportData {
    wastes: Vec<(Pos, Waste)>,
    vehicles: Vec<(Pos, Vehicle)>,
}

pub fn export_to_json(world: &World) -> String {
    let mut wastes = Vec::new();
    let mut vehicles = Vec::new();

    // Выборка всех waste
    for (_id, (pos, waste)) in world.query::<(&Pos, &Waste)>().iter() {
        wastes.push((pos.clone(), *waste));
    }

    // Выборка всех vehicle
    for (_id, (pos, vehicle)) in world.query::<(&Pos, &Vehicle)>().iter() {
        vehicles.push((pos.clone(), *vehicle));
    }

    let data = ExportData { wastes, vehicles };
    serde_json::to_string(&data).unwrap()
}
