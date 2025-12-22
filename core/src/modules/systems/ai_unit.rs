use hecs::{Entity, World};
use crate::descriptions::Descriptions;
use crate::modules::components::{Inventory, AttachedItems, BaseType, Guid, Pos, Target, Fraction, Health};
use crate::modules::markers::{Unit, Alert, IsMoving, IsWaitingTarget, Floor, IsTargetNear};
use crate::world_utils::attach_entity;
use std::collections::HashSet;

pub fn equip_items_from_inventory(world: &mut World, descriptions: &Descriptions, unit_entity: Entity) {
    // Получить base_type
    let base_type = match world.get::<&BaseType>(unit_entity) {
        Ok(bt) => bt.0.clone(),
        Err(_) => return,
    };

    // Получить slots_type из описания юнита
    let unit_desc = match descriptions.units.get(&base_type) {
        Some(desc) => desc,
        None => return,
    };

    let slots_type = match &unit_desc.slots_type {
        Some(st) => st,
        None => return, // Нет слотов
    };

    // Получить доступные слоты
    let available_slots = match descriptions.slots_types.get(slots_type) {
        Some(slots) => slots,
        None => return,
    };

    // Для каждого доступного слота
    for slot in available_slots {
        // Проверить, занят ли слот
        let is_slot_occupied = if let Ok(attached_items) = world.get::<&AttachedItems>(unit_entity) {
            attached_items.get(&slot.id).is_some()
        } else {
            false
        };

        if is_slot_occupied {
            continue;
        }

        // Найти предмет в инвентаре, подходящий для слота
        let mut item_to_equip = None;
        let mut item_index = None;

        if let Ok(inventory) = world.get::<&Inventory>(unit_entity) {
            for (idx, &item_entity) in inventory.0.iter().enumerate() {
                // Получить BaseType предмета
                if let Ok(base_type) = world.get::<&BaseType>(item_entity) {
                    // Получить описание предмета
                    if let Some(item_desc) = descriptions.items.get(&base_type.0) {
                        // Проверить tags
                        if let Some(item_tags) = &item_desc.tags {
                            // Проверить, есть ли пересечение tags
                            let compatible = item_tags.iter().any(|tag| slot.slot_tags.contains(tag));
                            if compatible {
                                item_to_equip = Some(item_entity);
                                item_index = Some(idx);
                                break;
                            }
                        }
                    }
                }
            }
        }

        // Если найден подходящий предмет, прикрепить
        if let (Some(item), Some(idx)) = (item_to_equip, item_index) {
            // Убедиться, что AttachedItems существует
            if world.get::<&AttachedItems>(unit_entity).is_err() {
                world.insert_one(unit_entity, AttachedItems::new()).unwrap();
            }
            // Прикрепить к слоту
            if let Ok(mut attached_items) = world.get::<&mut AttachedItems>(unit_entity) {
                attached_items.attach(&slot.id, item);
            }
            // Установить Owner для предмета
            if let Err(e) = attach_entity(world, item, unit_entity) {
                eprintln!("Failed to attach item to unit: {}", e);
                continue;
            }
            // Удалить из инвентаря
            if let Ok(mut inventory) = world.get::<&mut Inventory>(unit_entity) {
                inventory.0.remove(idx);
            }
            // Можно прикрепить только один предмет за раз, или продолжить для других слотов
        }
    }
}

fn set_target_to_waiting_units(world: &mut World, target_infos: &Vec<(hecs::Entity, Guid, Pos)>, floor_infos: &Vec<(hecs::Entity, Guid, Pos, Fraction)>, assigned_trash: &mut HashSet<hecs::Entity>) {
    let mut waiting_entities: Vec<(hecs::Entity, Target)> = Vec::new();

    for (entity, (pos, _unit, _waiting)) in
        world.query::<(&Pos, &Unit, &IsWaitingTarget)>()
        .iter()
    {
        // Check if unit is damaged
        let is_damaged = if let Ok(health) = world.get::<&Health>(entity) {
            health.current < health.max
        } else {
            false
        };

        let mut min_dist_sq = f32::INFINITY;
        let mut nearest = None;

        if is_damaged {
            // Seek nearest allied floor
            if let Ok(unit_faction) = world.get::<&Fraction>(entity) {
                for &(f_entity, f_guid, f_pos, f_fraction) in floor_infos {
                    if f_fraction != *unit_faction {
                        continue;
                    }
                    let dx = f_pos.x - pos.x;
                    let dy = f_pos.y - pos.y;
                    let dist_sq = dx * dx + dy * dy;
                    if dist_sq < min_dist_sq {
                        min_dist_sq = dist_sq;
                        nearest = Some((f_guid, f_entity));
                    }
                }
            }
        } else {
            // Seek nearest enemy target
            for &(t_entity, t_guid, t_pos) in target_infos {
                if assigned_trash.contains(&t_entity) || t_entity == entity {
                    continue;
                }
                // Skip targets with the same faction
                if let (Ok(unit_faction), Ok(target_faction)) = (world.get::<&Fraction>(entity), world.get::<&Fraction>(t_entity)) {
                    if *unit_faction == *target_faction {
                        continue;
                    }
                }
                let dx = t_pos.x - pos.x;
                let dy = t_pos.y - pos.y;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < min_dist_sq {
                    min_dist_sq = dist_sq;
                    nearest = Some((t_guid, t_entity));
                }
            }
        }

        if let Some((ng, ne)) = nearest {
            if !is_damaged {
                assigned_trash.insert(ne);
            }
            let target = Target { e: ne, guid: ng };
            waiting_entities.push((entity, target));
        }
    }

    for (entity, target) in waiting_entities {
        world.insert_one(entity, target).unwrap();
        world.insert_one(entity, IsMoving {}).unwrap();
        world.remove_one::<IsWaitingTarget>(entity).unwrap();
    }

    // For damaged units targeting floors, add IsTargetNear immediately since floors don't move
    let entities_to_add_near: Vec<hecs::Entity> = world.query::<(&Target, &Unit, &IsMoving, &Health)>().iter()
        .filter(|(_, (target, _, _, health))| {
            health.current < health.max &&
            if let Ok(target_entity_type) = world.get::<&crate::modules::components::EntityType>(target.e) {
                matches!(*target_entity_type, crate::modules::components::EntityType::Floor)
            } else {
                false
            }
        })
        .map(|(entity, _)| entity)
        .collect();

    for entity in entities_to_add_near {
        world.insert_one(entity, IsTargetNear {}).unwrap();
    }
}

fn reassign_closer_targets_for_moving_units(world: &mut World, trash_infos: &Vec<(hecs::Entity, Guid, Pos)>, floor_infos: &Vec<(hecs::Entity, Guid, Pos, Fraction)>, assigned_trash: &mut HashSet<hecs::Entity>) {
    let mut reassignments: Vec<(hecs::Entity, Target, hecs::Entity)> = Vec::new();

    for (entity, (pos, _unit, target, _moving)) in
        world.query::<(&Pos, &Unit, &Target, &IsMoving)>()
        .iter()
    {
        // Check if unit is damaged
        let is_damaged = if let Ok(health) = world.get::<&Health>(entity) {
            health.current < health.max
        } else {
            false
        };

        if let Ok(mut query) = world.query_one::<(&Pos,)>(target.e) {
            if let Some((current_pos,)) = query.get() {
                let dx_current = current_pos.x - pos.x;
                let dy_current = current_pos.y - pos.y;
                let current_dist_sq = dx_current * dx_current + dy_current * dy_current;

                let mut min_dist_sq = f32::INFINITY;
                let mut nearest = None;

                if is_damaged {
                    // Seek closer allied floor
                    if let Ok(unit_faction) = world.get::<&Fraction>(entity) {
                        for &(f_entity, f_guid, f_pos, f_fraction) in floor_infos {
                            if f_fraction != *unit_faction {
                                continue;
                            }
                            let dx = f_pos.x - pos.x;
                            let dy = f_pos.y - pos.y;
                            let dist_sq = dx * dx + dy * dy;
                            if dist_sq < min_dist_sq && dist_sq < current_dist_sq {
                                min_dist_sq = dist_sq;
                                nearest = Some((f_guid, f_entity));
                            }
                        }
                    }
                } else {
                    // Seek closer enemy target
                    for &(t_entity, t_guid, t_pos) in trash_infos {
                        if assigned_trash.contains(&t_entity) || t_entity == entity {
                            continue;
                        }
                        // Skip targets with the same faction
                        if let (Ok(unit_faction), Ok(target_faction)) = (world.get::<&Fraction>(entity), world.get::<&Fraction>(t_entity)) {
                            if *unit_faction == *target_faction {
                                continue;
                            }
                        }
                        let dx = t_pos.x - pos.x;
                        let dy = t_pos.y - pos.y;
                        let dist_sq = dx * dx + dy * dy;
                        if dist_sq < min_dist_sq && dist_sq < current_dist_sq {
                            min_dist_sq = dist_sq;
                            nearest = Some((t_guid, t_entity));
                        }
                    }
                }

                if let Some((ng, ne)) = nearest {
                    if !is_damaged {
                        assigned_trash.remove(&target.e);
                        assigned_trash.insert(ne);
                    }
                    let new_target = Target { e: ne, guid: ng };
                    reassignments.push((entity, new_target, target.e));
                }
            }
        }
    }

    for (unit_entity, new_target, _old_trash) in reassignments {
        world.remove_one::<Target>(unit_entity).unwrap();
        world.insert_one(unit_entity, new_target).unwrap();
    }
}

pub fn ai_unit_system(world: &mut World, descriptions: &Descriptions) {
    let unit_entities: Vec<Entity> = world.query::<&Unit>().iter().map(|(e, _)| e).collect();
    for unit_entity in unit_entities {
        equip_items_from_inventory(world, descriptions, unit_entity);
    }

    let mut target_infos: Vec<(hecs::Entity, Guid, Pos)> = Vec::new();
    for (entity, (pos, _alert, guid)) in world.query::<(&Pos, &Unit, &Guid)>().iter() {
        target_infos.push((entity, *guid, *pos));
    }

    for (entity, (pos, _alert, guid)) in world.query::<(&Pos, &Alert, &Guid)>().iter() {
        target_infos.push((entity, *guid, *pos));
    }

    let mut floor_infos: Vec<(hecs::Entity, Guid, Pos, Fraction)> = Vec::new();
    for (entity, (pos, _floor, guid, fraction)) in world.query::<(&Pos, &Floor, &Guid, &Fraction)>().iter() {
        floor_infos.push((entity, *guid, *pos, *fraction));
    }

    let mut assigned_entity: HashSet<hecs::Entity> = HashSet::new();
    for (_entity, target) in world.query::<&Target>().iter() {
        assigned_entity.insert(target.e);
    }

    set_target_to_waiting_units(world, &target_infos, &floor_infos, &mut assigned_entity);
    reassign_closer_targets_for_moving_units(world, &target_infos, &floor_infos, &mut assigned_entity);
}
