use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Force(pub f32);
