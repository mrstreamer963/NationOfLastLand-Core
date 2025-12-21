use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub const ZERO: Pos = Pos { x: 0.0, y: 0.0 };
    pub fn find_nearest_position(&self, positions: &[Pos]) -> Option<Pos> {
    let mut nearest: Option<Pos> = None;
    let mut min_distance_squared = f32::INFINITY;

    for &pos in positions {
        let dx = pos.x - self.x;
        let dy = pos.y - self.y;
        let distance_squared = dx * dx + dy * dy;

        if distance_squared < min_distance_squared {
            min_distance_squared = distance_squared;
            nearest = Some(pos);
        }
    }

    nearest
}

}
