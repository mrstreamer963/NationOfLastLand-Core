use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
