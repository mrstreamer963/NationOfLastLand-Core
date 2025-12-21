use crate::descriptions::Descriptions;
use crate::modules::components::{BaseType, EntityType, Pos, Reputation};
use crate::modules::markers::Base;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

pub fn create_base_from_description(world: &mut World, descriptions: &Descriptions, base_key: &str, pos: Pos) -> Result<Entity, String> {
    if let Some(description) = descriptions.bases.get(base_key) {
        let e = spawn_entity(world, (
            pos,
            Base {},
            EntityType::Base,
            BaseType(description.base_type.clone()),
            Reputation(description.reputation_cost_destroy)
        ));
        Ok(e)
    } else {
        Err(format!("Base '{}' not found in descriptions", base_key))
    }
}
