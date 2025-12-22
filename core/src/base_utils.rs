use crate::descriptions::Descriptions;
use crate::modules::components::{Inventory, Owner};
use crate::modules::markers::{AddFloorEvent, Item};
use crate::world_utils::{get_base_type, spawn_entity};
use hecs::{Entity, World};

pub fn can_attach_floor_to_base(
    world: &World,
    descriptions: &Descriptions,
    base: Entity,
    floor_type: &str,
) -> Result<(), String> {
    // Get base type
    let base_type = get_base_type(world, base)?;

    // Check if base type exists in descriptions
    let base_desc = descriptions.bases.get(&base_type)
        .ok_or(format!("Base type '{}' not found in descriptions", base_type))?;

    // Check if floor type exists in descriptions
    if !descriptions.units.contains_key(floor_type) {
        return Err(format!("Floor type '{}' not found in descriptions", floor_type));
    }

    // Count current floors attached to base
    let current_floors = world.query::<&Owner>()
        .iter()
        .filter(|(_, owner)| owner.e == base)
        .count();

    // Check if floors count is less than max_floors
    if current_floors >= base_desc.max_floors as usize {
        return Err(format!("Cannot attach floor: maximum floors ({}) reached for base type '{}'", base_desc.max_floors, base_type));
    }

    Ok(())
}

pub fn add_floor_to_base(
    world: &mut World,
    descriptions: &Descriptions,
    base: Entity,
    floor_type: &str,
) -> Result<(), String> {
    // Check if we can attach
    can_attach_floor_to_base(world, descriptions, base, floor_type)?;

    // Create attach event
    spawn_entity(world, (AddFloorEvent {
        base,
        floor_type: floor_type.to_string(),
    },));

    Ok(())
}

pub fn add_inventory(world: &mut World, entity: Entity, item_entity: Entity) -> Result<(), String> {
    // Check if item_entity has Item marker
    if world.get::<&Item>(item_entity).is_err() {
        return Err("item_entity is not an item".to_string());
    }

    // Get Inventory component for entity
    if let Ok(mut query) = world.query_one::<&mut Inventory>(entity) {
        if let Some(inventory) = query.get() {
            inventory.push(item_entity);
            Ok(())
        } else {
            Err("Entity does not have Inventory component".to_string())
        }
    } else {
        Err("Entity does not have Inventory component".to_string())
    }
}
