use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
#[serde(transparent)]
pub struct Reputation(pub f32);

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct ReputationCost(pub f32);
