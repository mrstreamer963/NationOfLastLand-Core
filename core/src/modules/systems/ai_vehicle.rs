use crate::modules::components::Pos;
use crate::modules::components::{
    EntityType, UnitState, MaxSpeed, TargetId, Velocity, Guid
};
use crate::modules::markers::Vehicle;
use crate::modules::world_state::WorldState;

use hecs::World;

fn move_vehicles(world: &mut World, ws: &WorldState) {
    // Query for vehicles that are moving and update their velocity and position
    for (_entity, (pos, target_id, velocity, max_speed, unit_state)) in world
        .query_mut::<(&mut Pos, &TargetId, &mut Velocity, &MaxSpeed, &mut UnitState)>()
    {
        if *unit_state == UnitState::IsMoving {
            // Find target position by Guid through entity
            if let Some(target_entity) = ws.guid_to_entity.get(&target_id.0) {
                if let Ok(mut query) = world.query_one::<&Pos>(*target_entity) {
                    if let Some(target_pos) = query.get() {
                        let target_pos = *target_pos;
                        let dx = target_pos.x - pos.x;
                        let dy = target_pos.y - pos.y;
                        let distance_squared = dx * dx + dy * dy;

                        // Threshold to consider reached, e.g., 1.0 units
                        const THRESHOLD: f32 = 0.1;
                        if distance_squared < THRESHOLD * THRESHOLD {
                            // Arrived at target
                            // Set position to target and reset velocity to zero
                            pos.x = target_pos.x;
                            pos.y = target_pos.y;
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
        }
    }
}

fn set_target_to_waiting_vehicles(world: &mut World) {
    // First, precompute all waste infos
    let mut trash_infos = Vec::new();
    for (_entity, (pos, unit_type, guid)) in world.query::<(&Pos, &EntityType, &Guid)>().iter() {
        if *unit_type == EntityType::Trash {
            trash_infos.push((*guid, *pos));
        }
    }

    // Then, collect all vehicle entities that are waiting for targets and their nearest waste
    let mut waiting_entities: Vec<(hecs::Entity, TargetId)> = Vec::new();

    for (entity, (pos, _vehicle, unit_state)) in
        world.query_mut::<(&Pos, &Vehicle, &mut UnitState)>()
    {
        if *unit_state == UnitState::IsWaitingTarget {
            // Find nearest trash by Guid
            let mut min_dist_sq = f32::INFINITY;
            let mut nearest_guid = None;
            for (guid, tpos) in &trash_infos {
                let dx = tpos.x - pos.x;
                let dy = tpos.y - pos.y;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < min_dist_sq {
                    min_dist_sq = dist_sq;
                    nearest_guid = Some(*guid);
                }
            }
            if let Some(ng) = nearest_guid {
                // Assign target
                let target = TargetId(ng);
                waiting_entities.push((entity, target));
                *unit_state = UnitState::IsMoving;
            }
        }
    }

    // Now add TargetId components to the entities
    for (entity, target) in waiting_entities {
        world.insert_one(entity, target).unwrap();
    }
}

fn attack_vehicles(_world: &mut World) {
    
}

/// System that processes vehicles waiting for targets, assigns nearest waste, and changes their state
pub fn ai_vehicle_system(world: &mut World, ws: &WorldState) {
    set_target_to_waiting_vehicles(world);

    move_vehicles(world, ws);

    attack_vehicles(world);
}
