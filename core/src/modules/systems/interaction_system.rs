use hecs::World;

use crate::{
    descriptions::Descriptions,
    modules::{components::{AttachedItems, DamageType, Target, WeaponMode}, markers::IsTargetNear},
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
