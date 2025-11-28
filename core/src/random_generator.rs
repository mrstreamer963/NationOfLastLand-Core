use rand::Rng;

use crate::{
    defines::{MapSize, MinMax},
    modules::{
        components::{DamageType, EntityType, Health, Pos, Resistance},
        markers::{Alert}
    },
};

pub struct RandomGenerator {
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
    pub fn get_bundle_trash(&self, map_size: &MapSize) -> (Pos, Health, EntityType, Alert, Resistance) {
        let pos = generate_random_pos(map_size);
        let health = generate_between(&self.toxic_health);
        let mut resistance = Resistance::default();
        resistance.resistances.insert(DamageType::Physical, 0.0);
        (pos, Health { current: health, max: health }, EntityType::Trash, Alert {}, resistance)
    }
}
