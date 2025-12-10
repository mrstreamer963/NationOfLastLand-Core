use crate::modules::components::{Guid, Pos};
use crate::modules::components::Target;
use crate::modules::markers::{IsMoving, IsWaitingTarget, Trash, Vehicle};
use std::collections::HashSet;
use hecs::World;

fn set_target_to_waiting_vehicles(world: &mut World, trash_infos: &Vec<(hecs::Entity, Guid, Pos)>, assigned_trash: &mut HashSet<hecs::Entity>) {
    // Collect all vehicle entities that are waiting for targets and their nearest waste
    let mut waiting_entities: Vec<(hecs::Entity, Target)> = Vec::new();

    for (entity, (pos, _vehicle, _waiting)) in
        world.query::<(&Pos, &Vehicle, &IsWaitingTarget)>()
        .iter()
    {
        // Find nearest trash by Entity and Guid that is not already assigned
        let mut min_dist_sq = f32::INFINITY;
        let mut nearest = None;
        for &(t_entity, t_guid, t_pos) in trash_infos {
            if assigned_trash.contains(&t_entity) {
                continue;
            }
            let dx = t_pos.x - pos.x;
            let dy = t_pos.y - pos.y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest = Some((t_guid, t_entity));
            }
        }
        if let Some((ng, ne)) = nearest {
            // Assign target
            assigned_trash.insert(ne);
            let target = Target { e: ne, guid: ng };
            waiting_entities.push((entity, target));
        }
    }

    for (entity, target) in waiting_entities {
        world.insert_one(entity, target).unwrap();
        world.insert_one(entity, IsMoving {}).unwrap();
        world.remove_one::<IsWaitingTarget>(entity).unwrap();
    }
}

fn reassign_closer_targets_for_moving_vehicles(world: &mut World, trash_infos: &Vec<(hecs::Entity, Guid, Pos)>, assigned_trash: &mut HashSet<hecs::Entity>) {
    let mut reassignments: Vec<(hecs::Entity, Target, hecs::Entity)> = Vec::new(); // vehicle, new_target, old_trash_to_remove

    for (entity, (pos, _vehicle, target, _moving)) in
        world.query::<(&Pos, &Vehicle, &Target, &IsMoving)>()
        .iter()
    {
        // Calculate distance to current target
        if let Ok(mut query) = world.query_one::<(&Pos,)>(target.e) {
            if let Some((current_pos,)) = query.get() {
                let dx_current = current_pos.x - pos.x;
                let dy_current = current_pos.y - pos.y;
                let current_dist_sq = dx_current * dx_current + dy_current * dy_current;

                // Find nearest trash closer than current
                let mut min_dist_sq = f32::INFINITY;
                let mut nearest = None;
                for &(t_entity, t_guid, t_pos) in trash_infos {
                    if assigned_trash.contains(&t_entity) {
                        continue; // Skip assigned trash
                    }
                    let dx = t_pos.x - pos.x;
                    let dy = t_pos.y - pos.y;
                    let dist_sq = dx * dx + dy * dy;
                    if dist_sq < min_dist_sq && dist_sq < current_dist_sq {
                        min_dist_sq = dist_sq;
                        nearest = Some((t_guid, t_entity));
                    }
                }
                if let Some((ng, ne)) = nearest {
                    // Free old trash and assign new
                    assigned_trash.remove(&target.e); // Remove old assignment
                    assigned_trash.insert(ne);
                    let new_target = Target { e: ne, guid: ng };
                    reassignments.push((entity, new_target, target.e));
                }
            }
        }
    }

    for (vehicle_entity, new_target, _old_trash) in reassignments {
        // Update the vehicle's target
        // Since target is a component, we can replace it
        world.remove_one::<Target>(vehicle_entity).unwrap();
        world.insert_one(vehicle_entity, new_target).unwrap();
    }
}

/// System that processes vehicles waiting for targets, assigns nearest waste, and changes their state
pub fn ai_vehicle_system(world: &mut World) {
    // First, precompute all waste infos
    let mut trash_infos: Vec<(hecs::Entity, Guid, Pos)> = Vec::new();
    for (entity, (pos, _trash, guid)) in world.query::<(&Pos, &Trash, &Guid)>().iter() {
        trash_infos.push((entity, *guid, *pos));
    }

    // Collect assigned trash: trash that are targets of vehicles (waiting, moving, or near target)
    let mut assigned_trash: HashSet<hecs::Entity> = HashSet::new();
    for (_entity, target) in world.query::<&Target>().iter() {
        assigned_trash.insert(target.e);
    }

    set_target_to_waiting_vehicles(world, &trash_infos, &mut assigned_trash);
    reassign_closer_targets_for_moving_vehicles(world, &trash_infos, &mut assigned_trash);
}
