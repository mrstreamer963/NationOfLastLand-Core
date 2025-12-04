use hecs::{World, Entity};
use crate::modules::components::BaseType;

pub fn get_base_type(world: &World, entity: Entity) -> Result<String, String> {
    if let Ok(mut query) = world.query_one::<&BaseType>(entity) {
        if let Some(base_type) = query.get() {
            Ok(base_type.0.clone())
        } else {
            Err("Vehicle has no BaseType component".to_string())
        }
    } else {
        Err("Vehicle not found".to_string())
    }
}
