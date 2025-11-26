use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Guid(pub u128);
