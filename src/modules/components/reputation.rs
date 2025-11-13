use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Reputation {
    pub value: i32,
}

impl Default for Reputation {
    fn default() -> Self {
        Self { value: 0 }
    }
}
