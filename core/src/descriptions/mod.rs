pub mod descriptions;
pub use descriptions::Descriptions;

use serde::Deserialize;
use serde_yaml;
use std::{error::Error, fs::File};

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
