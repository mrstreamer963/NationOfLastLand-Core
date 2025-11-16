use crate::modules::components::{IsWaitingTarget, Pos};
use crate::modules::entities::{Vehicle, Waste};
use hecs::World;

fn find_nearest_waste_from_list(waste_positions: &[Pos], from: Pos) -> Option<Pos> {
    let mut nearest: Option<Pos> = None;
    let mut min_distance_squared = f32::INFINITY;

    for &pos in waste_positions {
        let dx = pos.x - from.x;
        let dy = pos.y - from.y;
        let distance_squared = dx * dx + dy * dy;

        if distance_squared < min_distance_squared {
            min_distance_squared = distance_squared;
            nearest = Some(pos);
        }
    }

    nearest
}

/// System that finds nearest waste targets for waiting vehicles
pub fn ai_vehicle_system(world: &World) -> Vec<(hecs::Entity, Option<Pos>)> {
    // First, precompute all waste positions
    let mut waste_positions = Vec::new();
    for (_entity, (pos, _waste)) in world.query::<(&Pos, &Waste)>().iter() {
        waste_positions.push(*pos);
    }

    // Then, collect all vehicle entities that are waiting for targets
    let mut waiting_vehicles = Vec::new();
    for (entity, (pos, _vehicle, _waiting)) in
        world.query::<(&Pos, &Vehicle, &IsWaitingTarget)>().iter()
    {
        waiting_vehicles.push((entity, *pos));
    }

    // Find targets for each waiting vehicle
    let mut vehicle_targets = Vec::new();
    for (entity, pos) in waiting_vehicles {
        let nearest_waste = find_nearest_waste_from_list(&waste_positions, pos);
        vehicle_targets.push((entity, nearest_waste));
    }

    vehicle_targets
}
