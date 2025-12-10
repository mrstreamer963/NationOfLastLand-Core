use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

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
    pub interactions: Option<Vec<Interaction>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Interaction {
    pub name: String,
    #[serde(flatten)]
    pub effects: HashMap<String, f64>,
}

pub fn load_alerts_static(yaml: &str) -> Result<AlertsDescriptions, Box<dyn Error>> {
    let data: AlertsYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for alert in data.alerts {
        map.insert(alert.alert_type.clone(), alert);
    }
    Ok(map)
}
