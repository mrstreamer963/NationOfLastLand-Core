use crate::base_utils::add_inventory;
use crate::descriptions::Descriptions;
use crate::modules::components::{BaseType, EntityType, Fraction, Health, Inventory, Pos, Resistance, DamageType, UnitName};
use crate::modules::markers::{Unit, IsWaitingTarget};
use crate::random_generator::RandomGenerator;
use crate::spawner::create_item_from_description;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};
use rand::Rng;
use std::collections::HashMap;

pub fn create_unit_base(world: &mut World, descriptions: &Descriptions, unit_key: &str, pos: Pos, faction: Fraction, _r: &RandomGenerator) -> Result<Entity, String> {
    let unit_data = descriptions.units.get(unit_key)
        .ok_or(format!("Unit '{}' not found in descriptions", unit_key))?;

    // Spawn the entity with base components
    let e = spawn_entity(world, (
        BaseType(unit_key.to_string()),
        UnitName(unit_key.to_string()),
        pos,
        EntityType::Unit,
        Unit{},
        faction
    ));

    // Add health if max_health is specified
    if let Some(max_health_range) = &unit_data.max_health {
        let health = Health {
            current: max_health_range.min,
            max: max_health_range.min,
            cup: max_health_range.clone(),
        };
        world.insert_one(e, health).map_err(|_| "Failed to insert Health")?;
    }

    // Add resistance if specified
    if let Some(resistance_map) = &unit_data.resistance {
        let mut resistances = HashMap::new();
        for (key, value) in resistance_map {
            if let Some(damage_type) = DamageType::from_str(key) {
                resistances.insert(damage_type, *value as f32);
            } else {
                return Err(format!("Unknown damage type '{}'", key));
            }
        }
        let resistance = Resistance { resistances };
        world.insert_one(e, resistance).map_err(|_| "Failed to insert Resistance")?;
    }

    Ok(e)
}

pub fn fill_unit_inventory(world: &mut World, descriptions: &Descriptions, entity: Entity, _r: &RandomGenerator) -> Result<(), String> {
    let unit_key = {
        let base_type = world.get::<&BaseType>(entity).map_err(|_| "Entity does not have BaseType")?;
        base_type.0.clone()
    };
    let unit_data = descriptions.units.get(&unit_key)
        .ok_or(format!("Unit '{}' not found in descriptions", unit_key))?;

    // Add inventory if item_sets is specified
    if let Some(item_sets) = &unit_data.item_sets {
        if !item_sets.is_empty() {
            // Ensure the entity has an Inventory component
            if world.get::<&Inventory>(entity).is_err() {
                world.insert_one(entity, Inventory::new()).map_err(|_| "Failed to insert Inventory")?;
            }

            let mut rng = rand::thread_rng();
            let selected_set = &item_sets[rng.gen_range(0..item_sets.len())];
            for (_i, item_type) in selected_set.iter().enumerate() {
                let item_entity = create_item_from_description(world, descriptions, item_type, Pos::ZERO)?;
                add_inventory(world, entity, item_entity)?;
            }
        }
    }

    Ok(())
}

pub fn create_unit_from_description(world: &mut World, descriptions: &Descriptions, unit_key: &str, pos: Pos, faction: Fraction, r: &RandomGenerator) -> Result<Entity, String> {
    let e = create_unit_base(world, descriptions, unit_key, pos, faction, r)?;
    fill_unit_inventory(world, descriptions, e, r)?;

    // Add IsWaitingTarget if unit has weapons (item_sets is not empty)
    let unit_data = descriptions.units.get(unit_key)
        .ok_or(format!("Unit '{}' not found in descriptions", unit_key))?;
    if let Some(item_sets) = &unit_data.item_sets {
        if !item_sets.is_empty() {
            world.insert_one(e, IsWaitingTarget {}).map_err(|_| "Failed to insert IsWaitingTarget")?;
        }
    }

    Ok(e)
}
