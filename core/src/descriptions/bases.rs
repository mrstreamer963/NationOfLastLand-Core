use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

pub type BasesDescriptions = HashMap<String, BaseYaml>;

#[derive(Deserialize)]
pub struct BasesYaml {
    bases: Vec<BaseYaml>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BaseYaml {
    #[serde(rename = "type")]
    pub base_type: String,
    pub reputation_cost_destroy: f32,
    pub floors: Vec<String>,
    pub interactions: Option<Vec<Interaction>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Interaction {
    pub name: String,
    #[serde(flatten)]
    pub effects: HashMap<String, f64>,
}

pub fn load_bases_static(yaml: &str) -> Result<BasesDescriptions, Box<dyn Error>> {
    let data: BasesYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for base in data.bases {
        map.insert(base.base_type.clone(), base);
    }
    Ok(map)
}
