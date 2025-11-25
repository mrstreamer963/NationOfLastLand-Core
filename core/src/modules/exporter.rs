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

#[cfg(feature = "row-serialize")]
type UnitsType = Vec<Vec<Value>>;
#[cfg(not(feature = "row-serialize"))]
type UnitsType = Vec<HashMap<String, Value>>;

#[derive(Serialize, Deserialize)]
struct ExportData {
    units: UnitsType,
    state: State,
}

pub fn export_to_json(world: &World, state: &State) -> String {
    let mut units: UnitsType = Vec::new();

    let component_descriptions: Vec<(&str, Box<dyn Fn(&World, hecs::Entity) -> Option<Value>>)> = vec![
        ("health", Box::new(|world, id| world.get::<&Health>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("velocity", Box::new(|world, id| world.get::<&Velocity>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("rot", Box::new(|world, id| world.get::<&Rot>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("max_speed", Box::new(|world, id| world.get::<&MaxSpeed>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("target", Box::new(|world, id| world.get::<&TargetPos>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("reputation", Box::new(|world, id| world.get::<&Reputation>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("unit_state", Box::new(|world, id| world.get::<&UnitState>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("target_id", Box::new(|world, id| world.get::<&TargetId>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("damage_type", Box::new(|world, id| world.get::<&DamageType>(id).ok().map(|c| serde_json::to_value(*c).unwrap()))),
        ("alert", Box::new(|world, id| world.get::<&Alert>(id).ok().map(|_| Value::Bool(true)))),
        ("vehicle", Box::new(|world, id| world.get::<&Vehicle>(id).ok().map(|_| Value::Bool(true)))),
    ];

    // Выборка всех units с Pos и EntityType
    for (_id, (pos, entity_type)) in world.query::<(&Pos, &EntityType)>().iter() {
        let id_val = Value::Number(_id.id().into());
        let pos_val = serde_json::to_value(*pos).unwrap();
        let unit_type_val = serde_json::to_value(*entity_type).unwrap();

        #[cfg(feature = "row-serialize")]
        {
            let mut row = vec![id_val, pos_val, unit_type_val];
            for (name, getter) in &component_descriptions {
                if let Some(val) = getter(world, _id) {
                    row.push(val);
                } else if *name == "alert" || *name == "vehicle" {
                    row.push(Value::Bool(false));
                } else {
                    row.push(Value::Null);
                }
            }
            units.push(row);
        }

        #[cfg(not(feature = "row-serialize"))]
        {
            let mut map = HashMap::new();
            map.insert("id".to_string(), id_val);
            map.insert("pos".to_string(), pos_val);
            map.insert("unit_type".to_string(), unit_type_val);
            for (name, getter) in &component_descriptions {
                if let Some(val) = getter(world, _id) {
                    map.insert(name.to_string(), val);
                }
            }
            units.push(map);
        }
    }

    let data = ExportData {
        units,
        state: state.clone(),
    };
    serde_json::to_string(&data).unwrap()
}
