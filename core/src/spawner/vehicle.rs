use crate::descriptions::Descriptions;
use crate::modules::components::{BaseType, EntityType, Fraction, Force, Health, MaxSpeed, Pos, Rot, Velocity};
use crate::modules::markers::{IsWaitingTarget, Unit, Vehicle};
use crate::random_generator::RandomGenerator;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

pub fn create_vehicle_from_description(world: &mut World, descriptions: &Descriptions, vehicle_key: &str, pos: Pos, faction: Fraction, r: &RandomGenerator) -> Result<Entity, String> {
    let vehicle_data = descriptions.units.get(vehicle_key)
        .ok_or(format!("Vehicle '{}' not found in descriptions", vehicle_key))?;

    let max_speed = vehicle_data.max_speed.as_ref().ok_or(format!("Vehicle '{}' has no max_speed", vehicle_key))?;
    let max_health = vehicle_data.max_health.as_ref().ok_or(format!("Vehicle '{}' has no max_health", vehicle_key))?;

    let unit_name = r.generate_unit_name();

    let v =(
        BaseType(vehicle_key.to_string()),
        pos,
        Rot { x: 0.0, y: 0.0 },
        MaxSpeed(max_speed.clone()),
        Velocity { x: 0.0, y: 0.0 },
        Health { current: max_health.min, max: max_health.min, cup: max_health.clone() },
        Force(100.0),
        IsWaitingTarget {},
        EntityType::Unit,
        Unit{},
        Vehicle {},
        unit_name,
        faction
    );

    let e = spawn_entity(world, v);
    Ok(e)
}
