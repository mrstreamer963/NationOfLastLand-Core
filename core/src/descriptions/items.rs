use serde::{Deserialize, Deserializer};
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
    #[serde(default, deserialize_with = "deserialize_marker")]
    pub throwable: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_marker")]
    pub takeable: Option<bool>,
    #[serde(default)]
    pub interactions: Vec<ItemInteraction>,
}

#[derive(Deserialize, Debug)]
pub struct ItemInteraction {
    pub name: String,
    #[serde(flatten)]
    pub damage: HashMap<String, f64>,
}

// Keeping this for compatibility, but it might not be used anymore
#[derive(Deserialize, Debug)]
pub struct ItemAttackTypeYaml {
    #[serde(rename = "type")]
    pub attack_type: String,
    pub damage: f64,
}

fn deserialize_marker<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    // Since this function is only called if the field is present, return Some(true)
    // Consume the value to complete the deserialization
    let _ : Option<serde_yaml::Value> = Option::deserialize(deserializer)?;
    Ok(Some(true))
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
