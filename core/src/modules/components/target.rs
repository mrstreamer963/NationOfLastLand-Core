use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Target {
    pub pos: Option<super::Pos>,
}
