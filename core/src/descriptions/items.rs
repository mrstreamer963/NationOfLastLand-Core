use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

/// Структура для десериализации файла items.yml (список)
#[derive(Deserialize)]
struct ItemsYaml {
    items: Vec<ItemYaml>,
}

/// Структура для хранения предметов
#[derive(Debug, Default)]
pub struct ItemsContainer {
    pub items: HashMap<String, ItemYaml>,
}

#[derive(Deserialize, Debug)]
pub struct ItemYaml {
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(default)]
    pub attack_types: HashMap<String, Vec<ItemAttackTypeYaml>>,
}

#[derive(Deserialize, Debug)]
pub struct ItemAttackTypeYaml {
    #[serde(rename = "type")]
    pub attack_type: String,
    pub damage: f64,
}

/// Функция для получения предметов из статических данных
pub fn load_items_static(yaml: &str) -> Result<ItemsContainer, Box<dyn Error>> {
    let yaml_data: ItemsYaml = serde_yaml::from_str(yaml)?;
    let mut items_map = HashMap::new();
    for item in yaml_data.items {
        items_map.insert(item.item_type.clone(), item);
    }
    Ok(ItemsContainer { items: items_map })
}
