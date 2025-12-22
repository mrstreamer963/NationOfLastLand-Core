use hecs::World;

use crate::{
    descriptions::Descriptions,
    modules::{components::{AttachedItems, DamageType, Fraction, Pos, Target, WeaponMode}, markers::IsTargetNear},
    world_utils::{Attack, get_base_type, reset_target, spawn_attack_event}
};

// TODO сделать например вместо IsTargetNear - IsEnemyNear и переименовать на do_attack чтобы юниты не атаковали собственную базу например
pub fn do_interaction(world: &mut World, descriptions: &Descriptions) {
    let mut attack_events: Vec<Attack> = Vec::new();
    let mut entities_to_reset = Vec::new();

    for (entity, (_, target, attached_items)) in world
        .query::<(&IsTargetNear, &Target, &AttachedItems)>()
        .iter()
    {
        if !world.contains(target.e) {
            entities_to_reset.push(entity);
            continue;
        }

        // Check if target is an enemy (different faction)
        let should_attack = if let (Ok(attacker_fraction), Ok(target_fraction)) = (world.get::<&Fraction>(entity), world.get::<&Fraction>(target.e)) {
            *attacker_fraction != *target_fraction
        } else {
            // If either doesn't have faction, assume they can attack (for alerts/units without faction)
            true
        };

        if !should_attack {
            // Check if target is an allied floor
            let is_allied_floor = if let Ok(target_entity_type) = world.get::<&crate::modules::components::EntityType>(target.e) {
                matches!(*target_entity_type, crate::modules::components::EntityType::Floor)
            } else {
                false
            };

            if is_allied_floor {
                // Check distance to floor
                let distance = if let (Ok(unit_pos), Ok(floor_pos)) = (world.get::<&Pos>(entity), world.get::<&Pos>(target.e)) {
                    let dx = unit_pos.x - floor_pos.x;
                    let dy = unit_pos.y - floor_pos.y;
                    (dx * dx + dy * dy).sqrt()
                } else {
                    f32::INFINITY
                };

                println!("Applying floor repair to unit at distance {}", distance);
                // Apply floor's interactions to the unit (attacker)
                if let Ok(floor_base_type) = get_base_type(world, target.e) {
                    if let Some(floor_desc) = descriptions.units.get(&floor_base_type) {
                        if let Some(interactions) = &floor_desc.interactions {
                            for interaction in interactions {
                                let interaction_range = interaction.1.range.unwrap_or(0.0);
                                if distance <= interaction_range {
                                    for (damage_type_str, damage) in interaction.1.effects.iter() {
                                        if let Some(dt) = DamageType::from_str(damage_type_str) {
                                            let w = WeaponMode {
                                                damage_type: dt,
                                                damage: *damage as f32,
                                                range: interaction_range,
                                            };
                                            println!("Creating repair event: {} {}", damage_type_str, damage);
                                            // Create attack event where target is the unit (attacker)
                                            attack_events.push(Attack {
                                                weapon_mode: w, target_unit: entity });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Reset target for other non-enemy targets
                entities_to_reset.push(entity);
            }
            continue;
        }

        for (_key, item_entity) in attached_items.0.iter() {
            if let Ok(item_type) = get_base_type(world, *item_entity) {
                // println!("{}", item_type);
                if let Some(base_item) = descriptions.items.get(&item_type) {
                    if let Some(interactions) = &base_item.interactions {
                        for interaction in interactions {
                            for (damage_type_str, damage) in interaction.1.effects.iter() {
                                if let Some(dt) = DamageType::from_str(damage_type_str) {
                                    let w = WeaponMode {
                                        damage_type: dt,
                                        damage: *damage as f32,
                                        range: interaction.1.range.unwrap_or(0.0),
                                    };
                                    attack_events.push(Attack {
                                        weapon_mode: w, target_unit: target.e });
                                }
                            }
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
