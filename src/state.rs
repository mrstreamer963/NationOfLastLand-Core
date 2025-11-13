use crate::modules::components::Reputation;

pub struct State {
    pub r: Reputation,
}

impl Default for State {
    fn default() -> Self {
        Self {
            r: Reputation { value: 0 },
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
