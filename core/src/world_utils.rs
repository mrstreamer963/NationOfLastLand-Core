use hecs::{World, Entity};
use crate::modules::{components::{AttachedItems, BaseType, Guid, Inventory, Owner, Target, WeaponMode}, markers::{AttackEvent, IsTargetNear, IsWaitingTarget}};
use crate::internal_data::{add_guid_entity, get_guid_by_entity, remove_by_guid};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Attack {
    pub weapon_mode: WeaponMode,
    pub target_unit: Entity,
}

pub fn get_base_type(world: &World, entity: Entity) -> Result<String, String> {
    if let Ok(mut query) = world.query_one::<&BaseType>(entity) {
        if let Some(base_type) = query.get() {
            Ok(base_type.0.clone())
        } else {
            Err("Entity has no BaseType component".to_string())
        }
    } else {
        Err("Entity not found".to_string())
    }
}

pub fn spawn_entity(
    world: &mut World,
    bundle: impl hecs::Bundle + Send + Sync + 'static,
) -> hecs::Entity {
    let guid = Guid::new();
    let entity = world.spawn(bundle);
    world.insert_one(entity, guid).unwrap();

    // Update internal data maps
    if let Ok(guid) = world.get::<&Guid>(entity) {
        let guid = *guid;
        add_guid_entity(guid, entity);
    }

    entity
}

pub fn spawn_attack_event(world: &mut World, ev: Attack) -> Result<Entity, String> {
    let guid = world.query_one::<&Guid>(ev.target_unit).unwrap().get().unwrap().clone();
    let e = spawn_entity(world, (
        AttackEvent{},
        Target { e: ev.target_unit, guid },
        ev.weapon_mode
    ));

    Ok(e)
}

pub fn reset_target (world: &mut World, entity: Entity) {
    world.remove_one::<Target>(entity).unwrap();
    world.remove_one::<IsTargetNear>(entity).unwrap();
    world.insert_one(entity, IsWaitingTarget {}).unwrap();
}

pub fn attach_entity(world: &mut World, entity: Entity, owner: Entity) -> Result<(), String> {
    let owner_guid = get_guid_by_entity(&owner).ok_or("Owner guid not found in internal data".to_string())?;

    world.insert_one(entity, Owner { e: owner, guid: owner_guid }).map_err(|_| "Failed to insert Owner".to_string())?;
    Ok(())
}

pub fn remove_entity(world: &mut World, entity: Entity) -> Result<(), String> {
    if !world.contains(entity) {
        return Err("Entity not found".to_string());
    }

    // Collect entities to remove: owned entities, inventory items, attached items
    let mut entities_to_remove: HashSet<Entity> = HashSet::new();

    // Remove all owned entities
    let owned_entities: Vec<Entity> = world.query::<&Owner>().iter().filter_map(|(e, owner)| {
        if owner.e == entity {
            Some(e)
        } else {
            None
        }
    }).collect();
    entities_to_remove.extend(owned_entities);

    // Remove inventory items
    if let Ok(inventory) = world.get::<&Inventory>(entity) {
        entities_to_remove.extend(&inventory.0);
    }

    // Remove attached items
    if let Ok(attached_items) = world.get::<&AttachedItems>(entity) {
        for (_slot, item) in attached_items.0.iter() {
            entities_to_remove.insert(*item);
        }
    }

    // Remove collected entities
    for e in entities_to_remove {
        remove_entity(world, e)?;
    }

    // Remove from internal data maps
    if let Ok(guid) = world.get::<&Guid>(entity) {
        let guid = *guid;
        remove_by_guid(&guid);
    }

    // Despawn the entity
    world.despawn(entity).map_err(|_| "Failed to despawn entity".to_string())?;

    Ok(())
}
