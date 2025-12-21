use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

use crate::descriptions::InteractionDescriptions;

/// Структура для десериализации файла items.yml (список)
#[derive(Deserialize)]
struct ItemsYaml {
    items: Vec<ItemYaml>,
}

/// Структура для предметов
#[derive(Deserialize, Debug, Clone)]
pub struct ItemYaml {
    #[serde(rename = "type")]
    pub item_type: String,
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub takeable: Option<bool>,
    #[serde(default, deserialize_with = "crate::descriptions::interactions::deserialize_interactions")]
    pub interactions: Option<InteractionDescriptions>,
}

pub type ItemsDescriptions = HashMap<String, ItemYaml>;

/// Функция для получения предметов из статических данных
pub fn load_items_static(yaml: &str) -> Result<ItemsDescriptions, Box<dyn Error>> {
    let data: ItemsYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for item in data.items {
        map.insert(item.item_type.clone(), item);
    }
    Ok(map)
}
