use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

/// Структура для десериализации файла vehicles.yml (список)
#[derive(Deserialize)]
struct VehiclesYaml {
    vehicles: Vec<VehicleYaml>,
}

/// Структура для хранения транспортных средств
#[derive(Debug, Default)]
pub struct VehiclesContainer {
    pub vehicles: HashMap<String, VehicleYaml>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Slot {
    pub id: String,
    pub slot_type: String,
    pub mount_point: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VehicleYaml {
    #[serde(rename = "type")]
    pub vehicle_type: String,
    pub max_speed: crate::defines::MinMax,
    pub max_health: crate::defines::MinMax,
    #[serde(rename = "active_slot")]
    pub active_slot: Vec<Slot>,
    pub reputation_cost_buy: f32,
    pub reputation_cost_sell: f32,
    pub reputation_cost_destroy: f32,
}

/// Функция для получения транспортных средств из статических данных
pub fn load_vehicles_static(yaml: &str) -> Result<VehiclesContainer, Box<dyn Error>> {
    let yaml_data: VehiclesYaml = serde_yaml::from_str(yaml)?;
    let mut vehicles_map = HashMap::new();
    for vehicle in yaml_data.vehicles {
        vehicles_map.insert(vehicle.vehicle_type.clone(), vehicle);
    }
    Ok(VehiclesContainer { vehicles: vehicles_map })
}
