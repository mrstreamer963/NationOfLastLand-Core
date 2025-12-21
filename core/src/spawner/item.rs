use crate::descriptions::Descriptions;
use crate::modules::components::{BaseType, EntityType, Pos};
use crate::modules::markers::Item;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

pub fn create_item_from_description(world: &mut World, descriptions: &Descriptions, item_key: &str, pos: Pos) -> Result<Entity, String> {
    if let Some(_item_data) = descriptions.items.get(item_key) {
        let e = spawn_entity(world, (
            pos,
            BaseType(item_key.to_string()),
            EntityType::Item,
            Item {},
        ));

        Ok(e)
    } else {
        Err(format!("Item '{}' not found in descriptions", item_key))
    }
}
