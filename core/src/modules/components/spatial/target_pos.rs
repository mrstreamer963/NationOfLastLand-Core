use crate::modules::components::Pos;
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct TargetPos {
    pub value: Pos,
}
