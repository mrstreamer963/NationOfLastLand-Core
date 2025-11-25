use crate::modules::components::{EntityType, Health, Pos, TargetPos, Velocity};

use crate::modules::state::State;
use hecs::World;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ExportData {
    units: Vec<HashMap<String, Value>>,
    state: State,
}

pub fn export_to_json(world: &World, state: &State) -> String {
    let mut units = Vec::new();

    // Выборка всех alerts
    for (_id, (pos, alert_type )) in world.query::<(&Pos, &EntityType)>().iter() {
        let mut alert_data = HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
            ("unit_type".to_string(), serde_json::to_value(*alert_type).unwrap()),
        ]);

        if let Ok(health) = world.get::<&Health>(_id) {
            alert_data.insert("health".to_string(), serde_json::to_value(*health).unwrap());
        }

        // if let Ok(velocity) = world.get::<&Velocity>(_id) {
        //     alert_data.insert("velocity".to_string(), serde_json::to_value(*velocity).unwrap());
        // }

        // if let Ok(rot) = world.get::<&Velocity>(_id) {
        //     alert_data.insert("rot".to_string(), serde_json::to_value(*rot).unwrap());
        // }

        // if let Ok(max_speed) = world.get::<&Velocity>(_id) {
        //     alert_data.insert("max_speed".to_string(), serde_json::to_value(*max_speed).unwrap());
        // }
        
        // // Add optional components
        // if let Ok(target) = world.get::<&TargetPos>(_id) {
        //     alert_data.insert("target".to_string(), serde_json::to_value(*target).unwrap());
        // }

        units.push(alert_data);
    }

    let data = ExportData {
        units,
        state: state.clone(),
    };
    serde_json::to_string(&data).unwrap()
}
