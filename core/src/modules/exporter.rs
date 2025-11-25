use crate::modules::components::{DamageType, EntityType, Health, MaxSpeed, Pos, Reputation, Rot, TargetId, TargetPos, UnitState, Velocity};
use crate::modules::markers::{Alert, Vehicle};

use crate::modules::state::State;
use hecs::World;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

struct ComponentValue {
    name: String,
    value: Value,
}

impl ComponentValue {
    fn new<T: Serialize>(name: &str, component: &T) -> Self {
        Self {
            name: name.to_string(),
            value: serde_json::to_value(component).unwrap(),
        }
    }
}

type ComponentGetter = Box<dyn Fn(&World, hecs::Entity) -> Option<ComponentValue>>;

#[derive(Serialize, Deserialize)]
struct ExportData {
    units: Vec<HashMap<String, Value>>,
    state: State,
}

pub fn export_to_json(world: &World, state: &State) -> String {
    let mut units = Vec::new();

    let component_getters: Vec<ComponentGetter> = vec![
        Box::new(|world, id| world.get::<&Health>(id).ok().map(|c| ComponentValue::new("health", &*c))),
        Box::new(|world, id| world.get::<&Velocity>(id).ok().map(|c| ComponentValue::new("velocity", &*c))),
        Box::new(|world, id| world.get::<&Rot>(id).ok().map(|c| ComponentValue::new("rot", &*c))),
        Box::new(|world, id| world.get::<&MaxSpeed>(id).ok().map(|c| ComponentValue::new("max_speed", &*c))),
        Box::new(|world, id| world.get::<&TargetPos>(id).ok().map(|c| ComponentValue::new("target", &*c))),
        Box::new(|world, id| world.get::<&Reputation>(id).ok().map(|c| ComponentValue::new("reputation", &*c))),
        Box::new(|world, id| world.get::<&UnitState>(id).ok().map(|c| ComponentValue::new("unit_state", &*c))),
        Box::new(|world, id| world.get::<&TargetId>(id).ok().map(|c| ComponentValue::new("target_id", &*c))),
        Box::new(|world, id| world.get::<&DamageType>(id).ok().map(|c| ComponentValue::new("damage_type", &*c))),
        Box::new(|world, id| {
            world.get::<&Alert>(id).ok().map(|_| ComponentValue {
                name: "alert".to_string(),
                value: Value::Bool(true),
            })
        }),
        Box::new(|world, id| {
            world.get::<&Vehicle>(id).ok().map(|_| ComponentValue {
                name: "vehicle".to_string(),
                value: Value::Bool(true),
            })
        }),
    ];

    // Выборка всех alerts
    for (_id, (pos, alert_type )) in world.query::<(&Pos, &EntityType)>().iter() {
        let mut alert_data = HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
            ("unit_type".to_string(), serde_json::to_value(*alert_type).unwrap()),
        ]);

        for getter in &component_getters {
            if let Some(cv) = getter(world, _id) {
                alert_data.insert(cv.name, cv.value);
            }
        }

        units.push(alert_data);
    }

    let data = ExportData {
        units,
        state: state.clone(),
    };
    serde_json::to_string(&data).unwrap()
}
