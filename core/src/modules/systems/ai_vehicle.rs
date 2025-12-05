use crate::descriptions::{self, Descriptions};
use crate::modules::components::Pos;
use crate::modules::components::{MaxSpeed, TargetId, Velocity, Guid, Target, WeaponMode, AttachedItems};
use crate::modules::markers::{IsMoving, IsTargetNear, IsWaitingTarget, Trash, Vehicle};
use crate::modules::setup::Spatial;
use crate::world_utils::get_base_type;
use hecs::{Entity, World};

#[derive(Clone, Debug)]
pub struct AttackEvent {
    pub weapon_mode: WeaponMode,
    pub target_unit: Entity,
}

fn move_vehicles(world: &mut World, spatial: &Spatial) {
    let mut entities_to_stop = Vec::new();

    for (entity, (pos, target, velocity, max_speed, _)) in world
        .query::<(&mut Pos, &Target, &mut Velocity, &MaxSpeed, &IsMoving)>()
        .iter()
    {
        // Find target position directly by Entity
        let target_entity = target.0;
        if let Ok(mut query) = world.query_one::<(&Pos,)>(target_entity) {
            let (target_pos,) = query.get().unwrap();
            let dx = target_pos.x - pos.x;
            let dy = target_pos.y - pos.y;
            let distance_squared = dx * dx + dy * dy;

            // Threshold to consider reached, e.g., 1.0 units
            if distance_squared < spatial.threshold * spatial.threshold {
                // Arrived at target: set position to target and reset velocity to zero
                *pos = *target_pos;
                *velocity = Velocity { x: 0.0, y: 0.0 };
                entities_to_stop.push(entity);
            } else {
                // Move towards target: compute direction and set velocity
                let distance = distance_squared.sqrt();
                let dir_x = dx / distance;
                let dir_y = dy / distance;
                let new_vel_x = dir_x * max_speed.0;
                let new_vel_y = dir_y * max_speed.0;
                velocity.x = new_vel_x;
                velocity.y = new_vel_y;
                pos.x += new_vel_x;
                pos.y += new_vel_y;
            }
        }
    }

    // Change markers for stopped vehicles
    for entity in entities_to_stop {
        world.insert_one(entity, IsTargetNear {}).unwrap();
        world.remove_one::<IsMoving>(entity).unwrap();
    }
}

fn set_target_to_waiting_vehicles(world: &mut World) {
    // First, precompute all waste infos
    let mut trash_infos = Vec::new();
    for (entity, (pos, _trash, guid)) in world.query::<(&Pos, &Trash, &Guid)>().iter() {
        trash_infos.push((entity, *guid, *pos));
    }

    // Then, collect all vehicle entities that are waiting for targets and their nearest waste
    let mut waiting_entities: Vec<(hecs::Entity, TargetId, Target)> = Vec::new();

    for (entity, (pos, _vehicle, _waiting)) in
        world.query::<(&Pos, &Vehicle, &IsWaitingTarget)>()
        .iter()
    {
        // Find nearest trash by Entity and Guid
        let mut min_dist_sq = f32::INFINITY;
        let mut nearest_guid = None;
        let mut nearest_entity = None;
        for &(t_entity, t_guid, t_pos) in &trash_infos {
            let dx = t_pos.x - pos.x;
            let dy = t_pos.y - pos.y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest_guid = Some(t_guid);
                nearest_entity = Some(t_entity);
            }
        }
        if let (Some(ng), Some(ne)) = (nearest_guid, nearest_entity) {
            // Assign target
            let target_id = TargetId(ng);
            let target = Target(ne);
            waiting_entities.push((entity, target_id, target));
        }
    }

    // Now add TargetId and Target components to the entities and change state
    for (entity, target_id, target) in waiting_entities {
        world.insert_one(entity, target_id).unwrap();
        world.insert_one(entity, target).unwrap();
        world.insert_one(entity, IsMoving {}).unwrap();
        world.remove_one::<IsWaitingTarget>(entity).unwrap();
    }
}

fn attack_vehicles(world: &mut World, descriptions: &Descriptions) {
    let mut attack_vehicles: Vec<AttackEvent> = Vec::new();

    // let mut entities_to_reset = Vec::new();

    for (_entity, (_, _, target, attached_items)) in world
        .query::<(&IsTargetNear, &Vehicle, &Target, &AttachedItems)>()
        .iter()
    {
        for (_key, item_entity) in attached_items.0.iter() {
            if let Ok(item_type) = get_base_type(world, *item_entity) {
                // println!("{}", item_type);
                if let Some(base_item) = descriptions.items.get(&item_type) {
                    for interaction in base_item.interactions.iter() {

                    }
                }
            }

            // if let Ok(mut query) = world.query_one::<(&WeaponType,)>(*item_entity) {
            //     if let Some((weapon_type,)) = query.get() {
            //         for mode in &weapon_type.modes {
            //             attack_vehicles.push(AttackEvent {
            //                 weapon_mode: mode.clone(),
            //                 target_unit: target.0,
            //             });
            //         }
            //     }
            // }
        }
        // entities_to_reset.push(entity);
        // targets_to_despawn.push(target.0);
    }

    // Despawn targets
    // for target_entity in targets_to_despawn {
    //     world.despawn(target_entity).unwrap();
    // }

    // Reset vehicles to waiting state
    // for entity in entities_to_reset {
    //     world.insert_one(entity, IsWaitingTarget {}).unwrap();
    //     world.remove_one::<IsTargetNear>(entity).unwrap();
    //     world.remove_one::<Target>(entity).unwrap();
    //     world.remove_one::<TargetId>(entity).unwrap();
    // }
}

/// System that processes vehicles waiting for targets, assigns nearest waste, and changes their state
pub fn ai_vehicle_system(world: &mut World, spatial: &Spatial, descriptions: &Descriptions) {
    set_target_to_waiting_vehicles(world);

    move_vehicles(world, spatial);

    attack_vehicles(world, descriptions);
}
