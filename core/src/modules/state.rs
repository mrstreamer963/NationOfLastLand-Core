use crate::modules::components::Reputation;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct State {
    pub reputation: Reputation,
    pub time: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            reputation: Reputation { value: 0.0 },
            time: 0.0,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
