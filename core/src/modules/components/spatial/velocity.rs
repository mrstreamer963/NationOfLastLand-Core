use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
