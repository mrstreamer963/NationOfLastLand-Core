use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

#[derive(Deserialize, Debug)]
pub struct SlotTypeYaml {
    pub id: String,
    pub slot_tags: Vec<String>,
    pub mount_point: String,
}

// TODO порефакторить

#[derive(Deserialize)]
pub struct SlotsTypesYaml {
    pub slots_types: Vec<HashMap<String, Vec<SlotTypeYaml>>>,
}

pub fn load_slots_types_static(yaml: &str) -> Result<HashMap<String, Vec<SlotTypeYaml>>, Box<dyn Error>> {
    let data: SlotsTypesYaml = serde_yaml::from_str(yaml)?;
    let mut slots_types = HashMap::new();
    for map in data.slots_types {
        for (key, value) in map {
            slots_types.insert(key, value);
        }
    }
    Ok(slots_types)
}
