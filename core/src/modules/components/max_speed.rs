use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct MaxSpeed {
    pub value: f32,
}
