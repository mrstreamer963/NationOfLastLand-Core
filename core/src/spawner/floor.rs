use crate::descriptions::Descriptions;
use crate::modules::components::{BaseType, EntityType, Health};
use crate::modules::markers::Floor;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

pub fn create_floor_from_description(world: &mut World, descriptions: &Descriptions, floor_key: &str) -> Result<Entity, String> {
    let unit_data = descriptions.units.get(floor_key)
        .ok_or(format!("Floor '{}' not found in descriptions", floor_key))?;

    // Spawn the entity with base components
    let e = spawn_entity(world, (
        BaseType(floor_key.to_string()),
        EntityType::Floor,
        Floor {},
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

    Ok(e)
}
