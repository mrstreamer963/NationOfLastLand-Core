use hecs::World;
use crate::modules::state::State;
use crate::modules::markers::IsDead;
use crate::modules::components::{Reputation, BaseType};
use crate::descriptions::Descriptions;
use crate::world_utils::remove_entity;

pub fn do_remove_dead(world: &mut World, state: &mut State, descriptions: &Descriptions) {
    // Sum reputation from dead entities
    let mut reputation_to_add = 0.0;
    for (entity, _is_dead) in world.query::<&IsDead>().iter() {
        // If entity has Reputation component, use it
        if let Ok(reputation) = world.get::<&Reputation>(entity) {
            reputation_to_add += reputation.0;
        } else if let Ok(base_type) = world.get::<&BaseType>(entity) {
            // For units/vehicles, get reputation_cost_destroy from descriptions
            if let Some(unit_data) = descriptions.units.get(&base_type.0) {
                if let Some(cost) = unit_data.reputation_cost_destroy {
                    reputation_to_add += cost as f32;
                }
            }
        }
    }

    // Add to state reputation
    state.reputation.0 += reputation_to_add;

    // Collect entities with IsDead marker
    let mut entities_to_remove: Vec<hecs::Entity> = Vec::new();

    for (entity, _is_dead) in world.query::<&IsDead>().iter() {
        entities_to_remove.push(entity);
    }

    // Remove the collected entities
    for entity in entities_to_remove {
        remove_entity(world, entity).unwrap();
    }
}
