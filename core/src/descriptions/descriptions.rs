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

/// Функция для десериализации damage_types из статической строки YAML
pub fn load_damage_types_static(yaml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data: DamageTypesYaml = serde_yaml::from_str(yaml)?;
    Ok(data.damage_types)
}

/// Функция для получения предметов из статических данных
pub fn load_items_static(yaml: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let data: ItemsYaml = serde_yaml::from_str(yaml)?;
    let mut items = HashMap::new();
    for item in data.items {
        let description = format!(
            "Атаки: {}",
            item.attack_types
                .iter()
                .map(|at| format!("{} (урон: {})", at.attack_type, at.damage))
                .collect::<Vec<_>>()
                .join(", ")
        );
        items.insert(item.item_type, description);
    }
    Ok(items)
}

/// Компонент для хранения базовых описаний различных юнитов, алертов и предметов
#[derive(Debug, Default)]
pub struct Descriptions {
    /// Описания юнитов, где ключ - название юнита, значение - описание
    pub units: HashMap<String, String>,
    /// Описания алертов, где ключ - тип алерта, значение - описание
    pub alerts: HashMap<String, String>,
    /// Описания предметов, где ключ - название предмета, значение - описание
    pub items: HashMap<String, String>,
    /// Список типов повреждений
    pub damage_types: Vec<String>,
}
