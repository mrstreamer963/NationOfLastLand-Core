use hecs::{World, Entity};
use crate::modules::{components::{BaseType, Guid, Target, WeaponMode}, markers::{AttackEvent, IsTargetNear, IsWaitingTarget}};

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
            Err("Vehicle has no BaseType component".to_string())
        }
    } else {
        Err("Vehicle not found".to_string())
    }
}

pub fn spawn_entity(
    world: &mut World,
    bundle: impl hecs::Bundle + Send + Sync + 'static,
) -> hecs::Entity {
    let guid = Guid::new();
    let entity = world.spawn(bundle);
    world.insert_one(entity, guid).unwrap();
    entity
}

pub fn spawn_attack_event(world: &mut World, ev: Attack) -> Result<Entity, String> {
    let e = spawn_entity(world, (
        AttackEvent{},
        Target(ev.target_unit),
        ev.weapon_mode
    ));

    Ok(e)
}

pub fn reset_target (world: &mut World, entity: Entity) {
    world.remove_one::<Target>(entity).unwrap();
    world.remove_one::<IsTargetNear>(entity).unwrap();
    world.insert_one(entity, IsWaitingTarget {}).unwrap();
}

pub fn set_target (world: &mut World, entity: Entity, target: Entity) {

}
