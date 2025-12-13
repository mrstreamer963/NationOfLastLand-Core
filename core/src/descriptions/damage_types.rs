use serde::Deserialize;
use serde_yaml;
use std::error::Error;

/// Структура для десериализации файла damage_types.yml
#[derive(Deserialize)]
pub struct DamageTypesYaml {
    damage_types: Vec<String>,
}

/// Функция для десериализации damage_types из статической строки YAML
pub fn load_damage_types_static(yaml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data: DamageTypesYaml = serde_yaml::from_str(yaml)?;
    Ok(data.damage_types)
}
