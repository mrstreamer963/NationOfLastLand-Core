use hecs::{Entity, World};

use crate::modules::components::AttachedFloors;
use crate::modules::markers::AttachFloorEvent;
use crate::world_utils::attach_entity;

pub fn attach_process(world: &mut World) {
    // Collect attach events to process and remove them later to avoid borrowing issues
    let attach_events: Vec<(Entity, AttachFloorEvent)> = world
        .query::<&AttachFloorEvent>()
        .iter()
        .map(|(e, event)| (e, event.clone()))
        .collect();

    for (event_entity, event) in attach_events {
        // Attach floor to base
        if let Err(e) = attach_entity(world, event.floor, event.base) {
            eprintln!("Failed to attach floor to base: {}", e);
            continue;
        }

        // Add AttachedFloors to base if not present
        if world.get::<&AttachedFloors>(event.base).is_err() {
            world.insert_one(event.base, AttachedFloors::new()).unwrap();
        }

        // Add floor to AttachedFloors
        if let Ok(mut query) = world.query_one::<&mut AttachedFloors>(event.base) {
            if let Some(attached_floors) = query.get() {
                attached_floors.attach(event.floor);
            }
        }

        // Remove the attach event entity after processing
        world.despawn(event_entity).unwrap();
    }
}
