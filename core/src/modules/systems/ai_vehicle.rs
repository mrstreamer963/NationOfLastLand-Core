use crate::defines::Point;
use crate::modules::components::{
    UnitType, UnitState, MaxSpeed, Pos, TargetPos, Velocity
};

use hecs::World;

fn find_nearest_position_from_list(positions: &[Pos], from: Pos) -> Option<Pos> {
    let mut nearest: Option<Pos> = None;
    let mut min_distance_squared = f32::INFINITY;

    for &pos in positions {
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
    for (_entity, (pos, unit_type)) in world.query::<(&Pos, &UnitType)>().iter() {
        if matches!(unit_type, UnitType::Trash) {
            trash_positions.push(*pos);
        }
    }

    // Then, collect all vehicle entities that are waiting for targets
    // let mut waiting_entities = Vec::new();
    for (_entity, (pos, unit_type, unit_state)) in
        world.query_mut::<(&Pos, &UnitType, &mut UnitState)>()
    {
        if *unit_type == UnitType::Vehicle && *unit_state == UnitState::IsWaitingTarget {
            let nearest_waste = find_nearest_position_from_list(&trash_positions, *pos);
            if let Some(waste_pos) = nearest_waste {
                // Assign target
                let _target = TargetPos {
                    value: Point {
                        x: waste_pos.x,
                        y: waste_pos.y,
                    },
                };
                *unit_state = UnitState::IsMoving;
            }
        }
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
