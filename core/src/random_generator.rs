use rand::Rng;

use crate::{
    defines::{MapSize, MinMax},
    modules::{
        components::{EntityType, Health, Pos},
        markers::{Alert}
    },
};

pub struct RandomGenerator {
    pub size: MapSize,
    pub toxic_health: MinMax
}

pub fn generate_between(range: &MinMax) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range.min..=range.max)
}

pub fn generate_random_pos(map_size: &MapSize) -> Pos {
    let x_range = MinMax {
        min: 0.0,
        max: map_size.width as f32,
    };
    let y_range = MinMax {
        min: 0.0,
        max: map_size.height as f32,
    };
    Pos {
        x: generate_between(&x_range),
        y: generate_between(&y_range),
    }
}

impl RandomGenerator {
    pub fn create_trash(&self) -> (Pos, Health, EntityType, Alert) {
        let pos = generate_random_pos(&self.size);
        let health = generate_between(&self.toxic_health);
        (pos, Health(health), EntityType::Trash, Alert {})
    }
}
