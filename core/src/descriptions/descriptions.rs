use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

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
    pub attack_types: Vec<AttackTypeYaml>,
}

#[derive(Deserialize, Debug)]
pub struct AttackTypeYaml {
    #[serde(rename = "type")]
    pub attack_type: String,
    pub damage: f64,
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

/// Компонент для хранения базовых описаний различных юнитов, алертов и предметов
#[derive(Debug, Default)]
pub struct Descriptions {
    /// Описания юнитов, где ключ - название юнита, значение - описание
    pub units: HashMap<String, String>,
    /// Описания алертов, где ключ - тип алерта, значение - описание
    pub alerts: HashMap<String, String>,
    /// Предметы, где ключ - название предмета, значение - данные предмета
    pub items: HashMap<String, ItemYaml>,
    /// Список типов повреждений
    pub damage_types: Vec<String>,
}
