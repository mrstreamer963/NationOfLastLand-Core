use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

pub type FloorsDescriptions = HashMap<String, FloorYaml>;

#[derive(Deserialize)]
pub struct FloorsYaml {
    floors: Vec<FloorYaml>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FloorYaml {
    #[serde(rename = "type")]
    pub floor_type: String,
    pub interactions: Option<Vec<Interaction>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Interaction {
    pub name: String,
    #[serde(flatten)]
    pub effects: HashMap<String, f64>,
}

pub fn load_floors_static(yaml: &str) -> Result<FloorsDescriptions, Box<dyn Error>> {
    let data: FloorsYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for floor in data.floors {
        map.insert(floor.floor_type.clone(), floor);
    }
    Ok(map)
}
