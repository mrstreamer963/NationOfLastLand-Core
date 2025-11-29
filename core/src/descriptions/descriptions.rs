use std::collections::HashMap;

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
