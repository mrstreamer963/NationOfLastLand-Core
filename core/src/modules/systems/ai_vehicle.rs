use crate::descriptions::Descriptions;
use crate::modules::components::{Guid, Pos};
use crate::modules::components::{Target, WeaponMode, AttachedItems};
use crate::modules::markers::{IsMoving, IsTargetNear, IsWaitingTarget, Trash, Vehicle};
use crate::world_utils::{Attack, get_base_type, reset_target, spawn_attack_event};
use hecs::World;

fn set_target_to_waiting_vehicles(world: &mut World) {
    // First, precompute all waste infos
    let mut trash_infos = Vec::new();
    for (entity, (pos, _trash, guid)) in world.query::<(&Pos, &Trash, &Guid)>().iter() {
        trash_infos.push((entity, *guid, *pos));
    }

    // Then, collect all vehicle entities that are waiting for targets and their nearest waste
    let mut waiting_entities: Vec<(hecs::Entity, Target)> = Vec::new();

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
        if let (Some(_ng), Some(ne)) = (nearest_guid, nearest_entity) {
            // Assign target
            let target = Target(ne);
            waiting_entities.push((entity, target));
        }
    }

    for (entity, target) in waiting_entities {
        world.insert_one(entity, target).unwrap();
        world.insert_one(entity, IsMoving {}).unwrap();
        world.remove_one::<IsWaitingTarget>(entity).unwrap();
    }
}

fn interaction_vehicles(world: &mut World, descriptions: &Descriptions) {
    let mut attack_events: Vec<Attack> = Vec::new();
    let mut entities_to_reset = Vec::new();

    for (entity, (_, _, target, _attached_items)) in world
        .query::<(&IsTargetNear, &Vehicle, &Target, &AttachedItems)>()
        .iter()
    {
        if !world.contains(target.0) {
            entities_to_reset.push(entity);
            continue;
        }

        for (_key, item_entity) in _attached_items.0.iter() {
            if let Ok(item_type) = get_base_type(world, *item_entity) {
                // println!("{}", item_type);
                if let Some(base_item) = descriptions.items.get(&item_type) {
                    for interaction in base_item.interactions.iter() {
                        for (damage_type, damage) in interaction.action.iter() {
                            let w = WeaponMode {
                                damage_type: damage_type.clone(),
                                damage: *damage,
                                range: 0.0,
                            };
                            attack_events.push(Attack {
                                weapon_mode: w, target_unit: target.0 });
                        }
                    }
                }
            }
        }
    }

    for entity in entities_to_reset {
        reset_target(world, entity);
    }

    for e in attack_events {
        spawn_attack_event(world, e).expect("Can't insert attack event");
    }
}

/// System that processes vehicles waiting for targets, assigns nearest waste, and changes their state
pub fn ai_vehicle_system(world: &mut World, descriptions: &Descriptions) {
    set_target_to_waiting_vehicles(world);
    interaction_vehicles(world, descriptions);
}
