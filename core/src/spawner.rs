use crate::descriptions::Descriptions;
use crate::descriptions::alerts::AlertYaml;
use crate::descriptions::bases::BaseYaml;
use crate::modules::components::{BaseType, EntityType, Floors, Force, Health, MaxSpeed, Owner, Pos, Reputation, ReputationCost, Rot, Velocity};
use crate::modules::markers::{Base, Floor, IsWaitingTarget, Item, Vehicle};
use crate::random_generator::RandomGenerator;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

pub fn create_vehicle_from_description(world: &mut World, descriptions: &Descriptions, vehicle_key: &str, pos: Pos, r: &RandomGenerator) -> Result<Entity, String> {
    if let Some(vehicle_data) = descriptions.vehicles.get(vehicle_key) {
        let unit_name = r.generate_unit_name();
        let e = spawn_entity(world, (
            BaseType(vehicle_key.to_string()),
            pos,
            Rot { x: 0.0, y: 0.0 },
            MaxSpeed(vehicle_data.max_speed),
            Velocity { x: 0.0, y: 0.0 },
            Health { current: vehicle_data.max_health.min, max: vehicle_data.max_health.min, cup: vehicle_data.max_health },
            Force(100.0),
            IsWaitingTarget {},
            EntityType::Vehicle,
            Vehicle {},
            unit_name,
            Reputation(vehicle_data.reputation_cost_destroy),
            ReputationCost(vehicle_data.reputation_cost_buy)
        ));

        Ok(e)
    } else {
        Err(format!("Vehicle '{}' not found in descriptions", vehicle_key))
    }
}

// за interactions обращаемся каждый раз к словарю, для reload будет навешан компонент reload{ time, type }
pub fn create_item_from_description(world: &mut World, descriptions: &Descriptions, item_key: &str, pos: Pos) -> Result<Entity, String> {
    if let Some(_item_data) = descriptions.items.get(item_key) {
        let e = spawn_entity(world, (
            pos,
            BaseType(item_key.to_string()),
            EntityType::Item,
            Item {},
        ));

        Ok(e)
    } else {
        Err(format!("Item '{}' not found in descriptions", item_key))
    }
}

pub fn create_alert_from_description(world: &mut World, descriptions: &Descriptions, alert_key: &str, pos: Pos, r: &RandomGenerator) -> Result<Entity, String> {
    if let Some(description) = descriptions.alerts.get(alert_key) {
        match alert_key {
            "ALERT_TRASH" => Ok(create_trash(world, pos, r, description)),
            "ALERT_WASTE" => Ok(create_waste(world, pos, r, description)),
            _ => Err(format!("Unknown alert type '{}'", alert_key)),
        }
    } else {
        Err(format!("Alert '{}' not found in descriptions", alert_key))
    }
}

pub fn create_base_from_description(world: &mut World, descriptions: &Descriptions, base_key: &str, pos: Pos, r: &RandomGenerator) -> Result<Entity, String> {
    if let Some(description) = descriptions.bases.get(base_key) {
        match base_key {
            "BASE_MAIN" => Ok(create_main_base(world, pos, description)),
            "BASE_OUTPOST" => Ok(create_main_base(world, pos, description)),
            _ => Err(format!("Unknown base type '{}'", base_key)),
        }
    } else {
        Err(format!("Base '{}' not found in descriptions", base_key))
    }
}

pub fn create_floor_from_description(world: &mut World, descriptions: &Descriptions, floor_key: &str, pos: Pos, owner: Owner) -> Result<Entity, String> {
    if descriptions.floors.contains_key(floor_key) {
        let e = spawn_entity(world, (
            pos,
            BaseType(floor_key.to_string()),
            EntityType::Floor,
            Floor {},
            owner,
        ));

        Ok(e)
    } else {
        Err(format!("Floor '{}' not found in descriptions", floor_key))
    }
}

fn create_trash(world: &mut World, pos: Pos, r: &RandomGenerator, description: &AlertYaml) -> Entity {
    let bundle = r.get_bundle_trash(pos);
    let e = spawn_entity(world, bundle);
    world.insert_one(e, Reputation(description.reputation_cost_destroy)).unwrap();

    e
}

fn create_waste(world: &mut World, pos: Pos, r: &RandomGenerator,  description: &AlertYaml) -> Entity {
    let bundle = r.get_bundle_waste(pos);
    let e = spawn_entity(world, bundle);
    world.insert_one(e, Reputation(description.reputation_cost_destroy)).unwrap();

    e
}

fn create_main_base(world: &mut World, pos: Pos, description: &BaseYaml) -> Entity {
    let e = spawn_entity(world, (
        pos,
        Base {},
        EntityType::Base,
        BaseType(description.base_type.clone()),
        Reputation(description.reputation_cost_destroy),
        Floors(Vec::new())
    ));

    e
}
