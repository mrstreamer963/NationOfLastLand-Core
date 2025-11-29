use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Reputation {
    pub value: f32,
}
