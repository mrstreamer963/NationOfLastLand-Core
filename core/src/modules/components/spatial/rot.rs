use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Rot {
    pub x: f32,
    pub y: f32,
}
