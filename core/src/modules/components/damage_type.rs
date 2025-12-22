
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Clean,
    Acid,
    Physical,
    RepairForce,
}

impl DamageType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "DMG_CLEAN" => Some(DamageType::Clean),
            "DMG_ACID" => Some(DamageType::Acid),
            "Physical" => Some(DamageType::Physical),
            "REPAIR_FORCE" => Some(DamageType::RepairForce),
            _ => None,
        }
    }
}
