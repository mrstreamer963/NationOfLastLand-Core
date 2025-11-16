use crate::modules::components::{Pos, Target};
use crate::modules::entities::Vehicle;
use crate::ecs_utils::find_nearest_waste;
use hecs::World;

/// System that assigns the nearest waste target to each vehicle that doesn't already have one
pub fn assign_nearest_targets(world: &mut World) {
    // First, collect all vehicle entities and their positions that don't already have a target
    let mut vehicle_positions = Vec::new();
    for (entity, (pos, _vehicle)) in world.query::<(&Pos, &Vehicle)>().iter() {
        // Check if this vehicle already has a target
        if world.get::<&Target>(entity).is_err() {
            // No target component exists, so we can assign one
            vehicle_positions.push((entity, *pos));
        }
    }

    // Then find targets for each vehicle (immutable operations)
    let mut vehicle_targets = Vec::new();
    for (entity, pos) in vehicle_positions {
        let nearest_waste = find_nearest_waste(world, pos);
        vehicle_targets.push((entity, nearest_waste));
    }

    // Finally, assign targets to vehicles (mutable operations)
    for (entity, nearest_waste) in vehicle_targets {
        let target = Target { pos: nearest_waste };
        world.insert_one(entity, target).unwrap();
    }
}

/// System that finds all vehicles and determines their nearest waste targets
pub fn ai_vehicle_system(world: &World) -> Vec<(hecs::Entity, Option<Pos>)> {
    let mut vehicle_targets = Vec::new();

    // Query for all entities with Pos and Vehicle components
    for (entity, (pos, _vehicle)) in world.query::<(&Pos, &Vehicle)>().iter() {
        // Find the nearest waste for this vehicle
        let nearest_waste = find_nearest_waste(world, *pos);
        vehicle_targets.push((entity, nearest_waste));
    }

    vehicle_targets
}
