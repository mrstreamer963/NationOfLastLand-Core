use hecs::{World, Entity};
use crate::modules::{components::{BaseType, Guid, Owner, Target, WeaponMode}, markers::{AttackEvent, IsTargetNear, IsWaitingTarget}};

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
        crate::internal_data::INTERNAL_DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.guid_to_entity.insert(guid, entity);
            data.entity_to_guid.insert(entity, guid);
        });
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

pub fn attach_entity(world: &mut World, entity: Entity, owner: Entity) -> Result<(), String> {
    if !world.contains(entity) {
        return Err("Entity not found".to_string());
    }
    if !world.contains(owner) {
        return Err("Owner entity not found".to_string());
    }

    let owner_guid = crate::internal_data::INTERNAL_DATA.with(|data| {
        let data = data.borrow();
        data.entity_to_guid.get(&owner).copied()
    }).ok_or("Owner guid not found in internal data".to_string())?;

    world.insert_one(entity, Owner { e: owner, guid: owner_guid }).map_err(|_| "Failed to insert Owner".to_string())?;
    Ok(())
}

pub fn reset_target (world: &mut World, entity: Entity) {
    world.remove_one::<Target>(entity).unwrap();
    world.remove_one::<IsTargetNear>(entity).unwrap();
    world.insert_one(entity, IsWaitingTarget {}).unwrap();
}

pub fn remove_entity(world: &mut World, entity: Entity) -> Result<(), String> {
    if !world.contains(entity) {
        return Err("Entity not found".to_string());
    }

    // Remove all owned entities
    let owned_entities: Vec<Entity> = world.query::<&Owner>().iter().filter_map(|(e, owner)| {
        if owner.e == entity {
            Some(e)
        } else {
            None
        }
    }).collect();

    for owned in owned_entities {
        remove_entity(world, owned)?;
    }

    // Remove from internal data maps
    if let Ok(guid) = world.get::<&Guid>(entity) {
        let guid = *guid;
        crate::internal_data::INTERNAL_DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.guid_to_entity.remove(&guid);
            data.entity_to_guid.remove(&entity);
        });
    }

    // Despawn the entity
    world.despawn(entity).map_err(|_| "Failed to despawn entity".to_string())?;

    Ok(())
}
