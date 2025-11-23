use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct ToxicPower {
    pub level: f32,
}
