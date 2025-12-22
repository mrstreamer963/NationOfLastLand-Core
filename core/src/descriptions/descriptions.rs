// Re-export для обратной совместимости или удобства
pub use super::*;
use std::{collections::{HashMap, HashSet}, error::Error};

/// Компонент для хранения базовых описаний различных юнитов, алертов и предметов
#[derive(Debug, Default)]
pub struct Descriptions {
    /// Список тегов слотов
    pub slot_tags: Vec<String>,
    /// Типы слотов
    pub slots_types: HashMap<String, Vec<SlotTypeYaml>>,
    /// Список типов повреждений
    pub damage_types: Vec<String>,
    /// Описания алертов, где ключ - тип алерта, значение - структура с данными алерта
    pub alerts: AlertsDescriptions,
    /// Описания баз, где ключ - тип базы, значение - структура с данными базы
    pub bases: BasesDescriptions,
    /// Предметы, где ключ - название предмета, значение - данные предмета
    pub items: HashMap<String, ItemYaml>,
    /// Юниты и транспортные средства, где ключ - тип юнита/транспорта, значение - данные
    pub units: UnitsDescriptions,
}

impl Descriptions {
    /// Валидирует соответствия damage типов из предметов с damage_types
    pub fn validate_attack_types(&self) -> Result<(), Box<dyn Error>> {
        let valid_damage_types: HashSet<&String> =
            self.damage_types.iter().collect();

        for (item_name, item) in &self.items {
            if let Some(interactions) = &item.interactions {
                for interaction in interactions {
                    for damage_type in interaction.1.effects.keys() {
                        if !valid_damage_types.contains(damage_type) {
                            return Err(format!(
                                "Invalid attack type '{}' in item '{}'. Must match one of: {:?}",
                                damage_type, item_name, self.damage_types
                            ).into());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Валидирует что все slot_tags в slots_types содержатся в slot_tags
    pub fn validate_slot_tags(&self) -> Result<(), Box<dyn Error>> {
        let valid_slot_tags: HashSet<&String> = self.slot_tags.iter().collect();

        for (slots_type_name, slots) in &self.slots_types {
            for slot in slots {
                for tag in &slot.slot_tags {
                    if !valid_slot_tags.contains(tag) {
                        return Err(format!(
                            "Invalid slot tag '{}' in slot '{}' of slots_type '{}'. Must match one of: {:?}",
                            tag, slot.id, slots_type_name, self.slot_tags
                        ).into());
                    }
                }
            }
        }

        Ok(())
    }
}
