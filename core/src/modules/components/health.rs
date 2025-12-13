use serde::{Deserialize, Serialize};

use crate::defines::MinMax;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub cup: MinMax
}
