use crate::defines::Point;
use crate::modules::components::{
    EntityType, UnitState, MaxSpeed, Pos, TargetPos, Velocity
};
use crate::modules::markers::Vehicle;

use hecs::World;

fn move_vehicles(world: &mut World) {
    // Query for vehicles that are moving and update their velocity and position
    for (_entity, (pos, target, velocity, max_speed, unit_state)) in world
        .query_mut::<(&mut Pos, &TargetPos, &mut Velocity, &MaxSpeed, &mut UnitState)>()
    {
        if *unit_state == UnitState::IsMoving {
            let dx = target.value.x - pos.x;
            let dy = target.value.y - pos.y;
            let distance_squared = dx * dx + dy * dy;

            // Threshold to consider reached, e.g., 1.0 units
            const THRESHOLD: f32 = 0.1;
            if distance_squared < THRESHOLD * THRESHOLD {
                // Arrived at target
                // Set position to target and reset velocity to zero
                pos.x = target.value.x;
                pos.y = target.value.y;
                velocity.x = 0.0;
                velocity.y = 0.0;
                *unit_state = UnitState::IsStopped;
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
    }
}

fn set_target_to_waiting_vehicles(world: &mut World) {
    // First, precompute all waste positions
    let mut trash_positions = Vec::new();
    for (_entity, (pos, unit_type)) in world.query::<(&Pos, &EntityType)>().iter() {
        if *unit_type == EntityType::Trash {
            trash_positions.push(*pos);
        }
    }

    // Then, collect all vehicle entities that are waiting for targets and their nearest waste
    let mut waiting_entities: Vec<(hecs::Entity, TargetPos)> = Vec::new();
    
    for (entity, (pos, _vehicle, unit_state)) in
        world.query_mut::<(&Pos, &Vehicle, &mut UnitState)>()
    {
        if *unit_state == UnitState::IsWaitingTarget {
            let nearest_waste = pos.find_nearest_position(&trash_positions);
            if let Some(waste_pos) = nearest_waste {
                // Assign target
                let target = TargetPos {
                    value: Point {
                        x: waste_pos.x,
                        y: waste_pos.y,
                    },
                };
                waiting_entities.push((entity, target));
                *unit_state = UnitState::IsMoving;
            }
        }
    }

    // Now add TargetPos components to the entities
    for (entity, target) in waiting_entities {
        world.insert_one(entity, target).unwrap();
    }
}

fn attack_vehicles(_world: &mut World) {
    
}

/// System that processes vehicles waiting for targets, assigns nearest waste, and changes their state
pub fn ai_vehicle_system(world: &mut World) {
    set_target_to_waiting_vehicles(world);

    move_vehicles(world);

    attack_vehicles(world);
}
