use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Health(pub f32);
