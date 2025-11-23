use crate::defines::Point;
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct TargetPos {
    pub value: Point,
}
