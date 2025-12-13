use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Floors(pub Vec<String>);
