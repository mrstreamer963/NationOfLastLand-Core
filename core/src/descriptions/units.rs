use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

/// Структура для десериализации файла units.yml (список)
#[derive(Deserialize)]
struct UnitsYaml {
    units: Vec<UnitYaml>,
}

/// Структура для юнитов (включая транспортные средства)
#[derive(Deserialize, Debug, Clone)]
pub struct UnitYaml {
    #[serde(rename = "type")]
    pub unit_type: String,
    #[serde(default)]
    pub reputation_cost_destroy: Option<f64>,
    #[serde(default)]
    pub max_health: Option<crate::defines::MinMax>,
    #[serde(default)]
    pub slots_type: Option<String>,
    #[serde(default)]
    pub resistance: Option<HashMap<String, f64>>,
    #[serde(default)]
    pub item_sets: Option<Vec<Vec<String>>>,
    #[serde(default, deserialize_with = "crate::descriptions::interactions::deserialize_interactions")]
    pub interactions: Option<crate::descriptions::InteractionDescriptions>,
    // Поля для транспортных средств
    #[serde(default)]
    pub max_speed: Option<crate::defines::MinMax>,
    #[serde(default)]
    pub reputation_cost_buy: Option<f32>,
    #[serde(default)]
    pub reputation_cost_sell: Option<f32>,
    #[serde(default)]
    pub inventory: Option<Vec<String>>,
}

pub type UnitsDescriptions = HashMap<String, UnitYaml>;

/// Функция для получения юнитов из статических данных
pub fn load_units_static(yaml: &str) -> Result<UnitsDescriptions, Box<dyn Error>> {
    let data: UnitsYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for unit in data.units {
        map.insert(unit.unit_type.clone(), unit);
    }
    Ok(map)
}
