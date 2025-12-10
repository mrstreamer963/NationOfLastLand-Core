use hecs::World;
use crate::modules::state::State;
use crate::modules::markers::{IsDead, Alert};
use crate::modules::components::Reputation;

pub fn do_remove_dead(world: &mut World, state: &mut State) {
    // Sum reputation from dead alerts
    let mut reputation_to_add = 0.0;
    for (_entity, (reputation, _alert, _is_dead)) in world.query::<(&Reputation, &Alert, &IsDead)>().iter() {
        reputation_to_add += reputation.value;
    }

    // Add to state reputation
    state.reputation.value += reputation_to_add;

    // Collect entities with IsDead marker
    let mut entities_to_remove: Vec<hecs::Entity> = Vec::new();

    for (entity, _is_dead) in world.query::<&IsDead>().iter() {
        entities_to_remove.push(entity);
    }

    // Remove the collected entities
    for entity in entities_to_remove {
        world.despawn(entity).unwrap();
    }
}
