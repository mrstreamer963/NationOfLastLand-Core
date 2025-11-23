use crate::modules::components::{AlertType, Health, IsMoving, IsStopped, IsWaitingTarget, MaxSpeed, Pos, Rot, TargetPos, ToxicPower, Velocity};
use crate::modules::entities::Vehicle;
use crate::modules::state::State;
use hecs::World;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ExportData {
    alerts: Vec<HashMap<String, Value>>,
    vehicles: Vec<HashMap<String, Value>>,
    state: State,
}

pub fn export_to_json(world: &World, state: &State) -> String {
    let mut alerts = Vec::new();
    let mut vehicles = Vec::new();

    // Выборка всех alerts
    for (_id, (pos, health, alert_type )) in world.query::<(&Pos, &Health, &AlertType)>().iter() {
        let mut alert_data = HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
            ("health".to_string(), serde_json::to_value(health).unwrap()),
            ("alert_type".to_string(), serde_json::to_value(*alert_type).unwrap()),
        ]);

        // Add optional toxic_power
        if let Ok(toxic_power) = world.get::<&ToxicPower>(_id) {
            alert_data.insert("toxic_power".to_string(), serde_json::to_value(*toxic_power).unwrap());
        }

        alerts.push(alert_data);
    }

    // Выборка всех vehicle
    for (_id, (pos, rot, max_speed, _vehicle)) in world.query::<(&Pos, &Rot, &MaxSpeed, &Vehicle)>().iter() {
        let mut vehicle_data = HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
            ("rot".to_string(), serde_json::to_value(*rot).unwrap()),
            ("max_speed".to_string(), serde_json::to_value(*max_speed).unwrap()),
        ]);

        // Add optional components
        if let Ok(health) = world.get::<&Health>(_id) {
            vehicle_data.insert("health".to_string(), serde_json::to_value(*health).unwrap());
        }
        if let Ok(target) = world.get::<&TargetPos>(_id) {
            vehicle_data.insert("target".to_string(), serde_json::to_value(*target).unwrap());
        }
        if let Ok(velocity) = world.get::<&Velocity>(_id) {
            vehicle_data.insert("velocity".to_string(), serde_json::to_value(*velocity).unwrap());
        }

        // Add state
        let state = if world.get::<&IsWaitingTarget>(_id).is_ok() {
            "waiting"
        } else if world.get::<&IsMoving>(_id).is_ok() {
            "moving"
        } else if world.get::<&IsStopped>(_id).is_ok() {
            "stopped"
        } else {
            "unknown"
        };
        vehicle_data.insert("state".to_string(), Value::String(state.to_string()));

        vehicles.push(vehicle_data);
    }

    let data = ExportData {
        alerts,
        vehicles,
        state: state.clone(),
    };
    serde_json::to_string(&data).unwrap()
}
