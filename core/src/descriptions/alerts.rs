use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};
use crate::{defines::MinMax, descriptions::InteractionDescriptions};

pub type AlertsDescriptions = HashMap<String, AlertYaml>;

#[derive(Deserialize)]
pub struct AlertsYaml {
    alerts: Vec<AlertYaml>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AlertYaml {
    #[serde(rename = "type")]
    pub alert_type: String,
    pub reputation_cost_destroy: f32,
    pub max_health: MinMax,
    #[serde(default, deserialize_with = "crate::descriptions::interactions::deserialize_interactions")]
    pub interactions: Option<InteractionDescriptions>,
}

pub fn load_alerts_static(yaml: &str) -> Result<AlertsDescriptions, Box<dyn Error>> {
    let data: AlertsYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for alert in data.alerts {
        map.insert(alert.alert_type.clone(), alert);
    }
    Ok(map)
}
