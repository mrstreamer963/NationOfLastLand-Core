use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

use crate::defines::MinMax;
use crate::descriptions::InteractionDescriptions;

#[derive(Deserialize)]
pub struct FloorsYaml {
    floors: Vec<FloorYaml>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FloorYaml {
    #[serde(rename = "type")]
    pub floor_type: String,
    pub max_health: MinMax,
    #[serde(default, deserialize_with = "crate::descriptions::interactions::deserialize_interactions")]
    pub interactions: Option<InteractionDescriptions>,
}

pub type FloorsDescriptions = HashMap<String, FloorYaml>;

pub fn load_floors_static(yaml: &str) -> Result<FloorsDescriptions, Box<dyn Error>> {
    let data: FloorsYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for floor in data.floors {
        map.insert(floor.floor_type.clone(), floor);
    }
    Ok(map)
}
