use crate::descriptions::Descriptions;
use crate::descriptions::alerts::AlertYaml;
use crate::modules::components::{Pos, Reputation};
use crate::random_generator::RandomGenerator;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

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

fn create_trash(world: &mut World, pos: Pos, r: &RandomGenerator, description: &AlertYaml) -> Entity {
    let bundle = r.get_bundle_trash(pos, &description.max_health);
    let e = spawn_entity(world, bundle);
    world.insert_one(e, Reputation(description.reputation_cost_destroy)).unwrap();

    e
}

fn create_waste(world: &mut World, pos: Pos, r: &RandomGenerator,  description: &AlertYaml) -> Entity {
    let bundle = r.get_bundle_waste(pos, &description.max_health);
    let e = spawn_entity(world, bundle);
    world.insert_one(e, Reputation(description.reputation_cost_destroy)).unwrap();

    e
}
