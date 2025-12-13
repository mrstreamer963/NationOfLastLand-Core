use hecs::{Entity, World};

use crate::base_utils::can_attach_floor_to_base;
use crate::descriptions::Descriptions;
use crate::modules::components::AttachedFloors;
use crate::modules::markers::AddFloorEvent;
use crate::spawner::create_floor_from_description;
use crate::world_utils::attach_entity;

pub fn attach_process(world: &mut World, descriptions: &Descriptions) {
    // Collect attach events to process and remove them later to avoid borrowing issues
    let attach_events: Vec<(Entity, AddFloorEvent)> = world
        .query::<&AddFloorEvent>()
        .iter()
        .map(|(e, event)| (e, event.clone()))
        .collect();

    for (event_entity, event) in attach_events {
        // Check if we can still attach the floor (state might have changed since event creation)
        if let Err(e) = can_attach_floor_to_base(world, descriptions, event.base, &event.floor_type) {
            eprintln!("Cannot attach floor '{}' to base: {}", event.floor_type, e);
            // Remove the attach event entity
            world.despawn(event_entity).unwrap();
            continue;
        }

        // Create floor entity from description
        let floor_entity = match create_floor_from_description(world, descriptions, &event.floor_type) {
            Ok(entity) => entity,
            Err(e) => {
                eprintln!("Failed to create floor '{}': {}", event.floor_type, e);
                // Remove the attach event entity
                world.despawn(event_entity).unwrap();
                continue;
            }
        };

        // Attach floor to base
        if let Err(e) = attach_entity(world, floor_entity, event.base) {
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
                attached_floors.attach(floor_entity);
            }
        }

        // Remove the attach event entity after processing
        world.despawn(event_entity).unwrap();
    }
}
