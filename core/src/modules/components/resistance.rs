use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::damage_type::DamageType;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Resistance {
    pub resistances: HashMap<DamageType, f32>,
}
