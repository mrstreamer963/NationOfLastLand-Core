pub mod descriptions;
pub use descriptions::Descriptions;

use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error, fs::File};

/// Структура для десериализации файла damage_types.yml
#[derive(Deserialize)]
pub struct DamageTypesYaml {
    damage_types: Vec<String>,
}

/// Функция для десериализации файла damage_types.yml
pub fn load_damage_types(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let data: DamageTypesYaml = serde_yaml::from_reader(file)?;
    Ok(data.damage_types)
}

/// Структура для десериализации файла items.yml
#[derive(Deserialize)]
pub struct ItemsYaml {
    items: Vec<ItemYaml>,
}

#[derive(Deserialize)]
pub struct ItemYaml {
    #[serde(rename = "type")]
    item_type: String,
    attack_types: Vec<AttackTypeYaml>,
}

#[derive(Deserialize)]
pub struct AttackTypeYaml {
    #[serde(rename = "type")]
    attack_type: String,
    damage: f64,
}

/// Функция для десериализации файла items.yml
pub fn load_items(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let data: ItemsYaml = serde_yaml::from_reader(file)?;
    let mut items = HashMap::new();
    for item in data.items {
        let description = format!("Атаки: {}", item.attack_types.iter()
            .map(|at| format!("{} (урон: {})", at.attack_type, at.damage))
            .collect::<Vec<_>>()
            .join(", "));
        items.insert(item.item_type, description);
    }
    Ok(items)
}
