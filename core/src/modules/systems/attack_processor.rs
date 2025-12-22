use hecs::{Entity, World};

use crate::modules::components::{Health, Target, WeaponMode, Resistance};
use crate::modules::markers::{AttackEvent, IsDead};

pub fn attack_process(world: &mut World) {
    // Collect attack events to process and remove them later to avoid borrowing issues
    let attack_events: Vec<(Entity, Target, WeaponMode)> = world
        .query::<(&Target, &WeaponMode, &AttackEvent)>()
        .iter()
        .map(|(e, (target, weapon_mode, _))| (e, target.clone(), weapon_mode.clone()))
        .collect();

    for (e, target, weapon_mode) in attack_events {
        // println!("attack_events...");
        // Get the target's health and resistance
        let mut should_add_dead_marker = false;
        if let Ok(mut query) = world.query_one::<(&mut Health, Option<&Resistance>)>(target.e) {
            if let Some((health, resistance_opt)) = query.get() {
                let mut damage = weapon_mode.damage;

                // Apply resistance if present
                if let Some(resistance) = resistance_opt {
                    let resistance_value = resistance.resistances.get(&weapon_mode.damage_type).unwrap_or(&0.0);
                    damage *= 1.0 - *resistance_value;
                }

                // Deal damage or heal
                if weapon_mode.damage_type == crate::modules::components::DamageType::RepairForce {
                    health.current += damage; // Heal
                } else {
                    health.current -= damage; // Damage
                }

                // Ensure health doesn't go below zero and doesn't exceed max
                if health.current < 0.0 {
                    health.current = 0.0;
                } else if health.current > health.max {
                    health.current = health.max;
                }

                // Check if dead
                if health.current == 0.0 {
                    should_add_dead_marker = true;
                }
            }
        }

        // Add IsDead marker if health reached zero
        if should_add_dead_marker {
            world.insert_one(target.e, IsDead {}).unwrap();
        }

        // Remove the attack event entity after processing
        world.despawn(e).unwrap();
    }
}
