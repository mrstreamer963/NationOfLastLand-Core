use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct MapSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Clone, Copy)]
pub struct MinMax {
    pub max: f32,
    pub min: f32,
}
