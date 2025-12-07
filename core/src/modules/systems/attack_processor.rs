use hecs::{Entity, World};

use crate::descriptions::Descriptions;
use crate::modules::components::{Health, Target, WeaponMode, Resistance};
use crate::modules::markers::AttackEvent;

pub fn attack_process(world: &mut World, _descriptions: &Descriptions) {
    // Collect attack events to process and remove them later to avoid borrowing issues
    let attack_events: Vec<(Entity, Target, WeaponMode)> = world
        .query::<(&Target, &WeaponMode, &AttackEvent)>()
        .iter()
        .map(|(e, (target, weapon_mode, _))| (e, target.clone(), weapon_mode.clone()))
        .collect();

    for (e, target, weapon_mode) in attack_events {
        println!("attack_events...");
        // Get the target's health and resistance
        if let Ok(mut query) = world.query_one::<(&mut Health, Option<&Resistance>)>(target.0) {
            if let Some((health, resistance_opt)) = query.get() {
                let mut damage = weapon_mode.damage as f32;

                // Apply resistance if present
                if let Some(resistance) = resistance_opt {
                    let resistance_value = resistance.resistances.get(&weapon_mode.damage_type).unwrap_or(&0.0);
                    damage *= 1.0 - *resistance_value;
                }

                // Deal damage
                health.current -= damage;

                // Ensure health doesn't go below zero
                if health.current < 0.0 {
                    health.current = 0.0;
                }

                println!("health {}", health.current);

                // Note: Entity removal or death handling can be added later if needed
            }
        }

        // Remove the attack event entity after processing
        world.despawn(e).unwrap();
    }
}
