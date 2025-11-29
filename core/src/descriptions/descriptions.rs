use serde::Deserialize;
use serde_yaml;
use std::{collections::{HashMap, HashSet}, error::Error};

/// Структура для десериализации файла damage_types.yml
#[derive(Deserialize)]
pub struct DamageTypesYaml {
    damage_types: Vec<String>,
}

/// Структура для десериализации файла items.yml
#[derive(Deserialize)]
pub struct ItemsContainer {
    pub items: HashMap<String, ItemYaml>,
}

#[derive(Deserialize, Debug)]
pub struct ItemYaml {
    pub attack_types: HashMap<String, Vec<ItemAttackTypeYaml>>,
}

#[derive(Deserialize, Debug)]
pub struct ItemAttackTypeYaml {
    #[serde(rename = "type")]
    pub attack_type: String,
    pub damage: f64,
}

/// Структура для десериализации файла vehicles.yml
#[derive(Deserialize)]
pub struct VehiclesContainer {
    pub vehicles: HashMap<String, VehicleYaml>,
}

#[derive(Deserialize, Debug)]
pub struct VehicleYaml {
    pub max_speed: f64,
    pub health: HealthYaml,
}

#[derive(Deserialize, Debug)]
pub struct HealthYaml {
    pub current: f64,
    pub max: f64,
}

/// Функция для десериализации damage_types из статической строки YAML
pub fn load_damage_types_static(yaml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data: DamageTypesYaml = serde_yaml::from_str(yaml)?;
    Ok(data.damage_types)
}

/// Функция для получения предметов из статических данных
pub fn load_items_static(yaml: &str) -> Result<ItemsContainer, Box<dyn Error>> {
    Ok(serde_yaml::from_str(yaml)?)
}

/// Функция для получения транспортных средств из статических данных
pub fn load_vehicles_static(yaml: &str) -> Result<VehiclesContainer, Box<dyn Error>> {
    Ok(serde_yaml::from_str(yaml)?)
}

/// Компонент для хранения базовых описаний различных юнитов, алертов и предметов
#[derive(Debug, Default)]
pub struct Descriptions {
    /// Описания юнитов, где ключ - название юнита, значение - описание
    pub units: HashMap<String, String>,
    /// Описания алертов, где ключ - тип алерта, значение - описание
    pub alerts: HashMap<String, String>,
    /// Предметы, где ключ - название предмета, значение - данные предмета
    pub items: HashMap<String, ItemYaml>,
    /// Транспортные средства, где ключ - название транспорта, значение - данные транспорта
    pub vehicles: HashMap<String, VehicleYaml>,
    /// Список типов повреждений
    pub damage_types: Vec<String>,
}
 
impl Descriptions {
    /// Валидирует соответствия attack_types.type из предметов с damage_types
    pub fn validate_attack_types(&self) -> Result<(), Box<dyn Error>> {
        let valid_damage_types: HashSet<&String> =
            self.damage_types.iter().collect();

        for (item_name, item) in &self.items {
            for (_attack_name, damages) in &item.attack_types {
                for attack in damages {
                    if !valid_damage_types.contains(&attack.attack_type) {
                        return Err(format!(
                            "Invalid attack type '{}' in item '{}'. Must match one of: {:?}",
                            attack.attack_type, item_name, self.damage_types
                        ).into());
                    }
                }
            }
        }

        Ok(())
    }
}
