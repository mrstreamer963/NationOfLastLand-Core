use crate::defines::Point;
use crate::modules::components::{
    IsMoving, IsStopped, IsWaitingTarget, MaxSpeed, Pos, Target, Velocity,
};
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

fn move_vehicles(world: &mut World) {
    // Collect entities that have arrived
    let mut arrived_entities = Vec::new();

    // Query for vehicles that are moving and update their velocity and position
    for (entity, (pos, target, velocity, max_speed, _is_moving)) in world
        .query::<(&mut Pos, &Target, &mut Velocity, &MaxSpeed, &IsMoving)>()
        .iter()
    {
        let dx = target.value.x - pos.x;
        let dy = target.value.y - pos.y;
        let distance_squared = dx * dx + dy * dy;

        // Threshold to consider reached, e.g., 1.0 units
        const THRESHOLD: f32 = 0.1;
        if distance_squared < THRESHOLD * THRESHOLD {
            // Arrived at target
            arrived_entities.push(entity);
            // Set position to target and reset velocity to zero
            pos.x = target.value.x;
            pos.y = target.value.y;
            velocity.x = 0.0;
            velocity.y = 0.0;
        } else {
            // Compute direction and set velocity
            let distance = distance_squared.sqrt();
            let dir_x = dx / distance;
            let dir_y = dy / distance;
            velocity.x = dir_x * max_speed.value;
            velocity.y = dir_y * max_speed.value;
            // Move the vehicle
            pos.x += velocity.x;
            pos.y += velocity.y;
        }
    }

    // Change state for arrived vehicles
    for entity in arrived_entities {
        world.remove_one::<IsMoving>(entity).unwrap();
        world.insert_one(entity, IsStopped {}).unwrap();
    }
}

fn set_target_to_waiting_vehicles(world: &mut World) {
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

    // Find targets for each waiting vehicle and assign them
    for (entity, pos) in waiting_vehicles {
        let nearest_waste = find_nearest_waste_from_list(&waste_positions, pos);
        if let Some(waste_pos) = nearest_waste {
            // Assign target
            let target = Target {
                value: Point {
                    x: waste_pos.x,
                    y: waste_pos.y,
                },
            };
            world.insert_one(entity, target).unwrap();

            // Remove waiting state
            world.remove_one::<IsWaitingTarget>(entity).unwrap();

            // Add moving state
            world.insert_one(entity, IsMoving {}).unwrap();
        }
    }
}

/// System that processes vehicles waiting for targets, assigns nearest waste, and changes their state
pub fn ai_vehicle_system(world: &mut World) {
    set_target_to_waiting_vehicles(world);

    move_vehicles(world);
}
