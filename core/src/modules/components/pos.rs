use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}
