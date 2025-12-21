use crate::descriptions::Descriptions;
use crate::modules::components::{BaseType, EntityType};
use crate::modules::markers::Floor;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

pub fn create_floor_from_description(world: &mut World, descriptions: &Descriptions, floor_key: &str) -> Result<Entity, String> {
    if let Some(_floor_data) = descriptions.floors.get(floor_key) {
        let e = spawn_entity(world, (
            BaseType(floor_key.to_string()),
            EntityType::Floor,
            Floor {},
        ));

        Ok(e)
    } else {
        Err(format!("Floor '{}' not found in descriptions", floor_key))
    }
}
