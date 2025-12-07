// Re-export для обратной совместимости или удобства
pub use super::*;
use std::{collections::{HashMap, HashSet}, error::Error};

/// Компонент для хранения базовых описаний различных юнитов, алертов и предметов
#[derive(Debug, Default)]
pub struct Descriptions {
    /// Описания алертов, где ключ - тип алерта, значение - описание
    pub alerts: AlertsDescriptions,
    /// Предметы, где ключ - название предмета, значение - данные предмета
    pub items: HashMap<String, ItemYaml>,
    /// Транспортные средства, где ключ - название транспорта, значение - данные транспорта
    pub vehicles: HashMap<String, VehicleYaml>,
    /// Список типов повреждений
    pub damage_types: Vec<String>,
}
 
impl Descriptions {
    /// Валидирует соответствия damage типов из предметов с damage_types
    pub fn validate_attack_types(&self) -> Result<(), Box<dyn Error>> {
        let valid_damage_types: HashSet<&String> =
            self.damage_types.iter().collect();

        for (item_name, item) in &self.items {
            for interaction in &item.interactions {
                for action_type in interaction.action.keys() {
                    if !valid_damage_types.contains(action_type) {
                        return Err(format!(
                            "Invalid attack type '{}' in item '{}'. Must match one of: {:?}",
                            action_type, item_name, self.damage_types
                        ).into());
                    }
                }
            }
        }

        Ok(())
    }
}
