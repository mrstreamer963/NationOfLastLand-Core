use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}
