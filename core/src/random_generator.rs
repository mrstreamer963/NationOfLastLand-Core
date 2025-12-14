use rand::Rng;

use crate::{
    defines::{MapSize, MinMax},
    modules::{
        components::{DamageType, EntityType, Health, Pos, Resistance, UnitName},
        markers::{Alert, Trash}
    },
};

pub struct RandomGenerator {
    pub trash_probability_threshold: f32,
    pub waste_probability_threshold: f32,
}

pub fn generate_between(range: &MinMax) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range.min..=range.max)
}

pub fn generate_probability() -> f32 {
    rand::random()
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
    pub fn get_bundle_trash(&self, pos: Pos, max_health: &MinMax) -> (Pos, Health, EntityType, Alert, Trash, Resistance) {
        let health = generate_between(max_health);
        let mut resistance = Resistance::default();
        resistance.resistances.insert(DamageType::Physical, 0.0);
        (pos, Health { current: health, max: health, cup: MinMax { max: health, min: health } }, EntityType::Trash, Alert {}, Trash {}, resistance)
    }

    pub fn get_bundle_waste(&self, pos: Pos, max_health: &MinMax) -> (Pos, Health, EntityType, Alert, Resistance) {
        let health = generate_between(max_health);
        let mut resistance = Resistance::default();
        resistance.resistances.insert(DamageType::Physical, 0.0);
        (pos, Health { current: health, max: health, cup: MinMax { max: health, min: health } }, EntityType::Waste, Alert {}, resistance)
    }

    pub fn generate_unit_name(&self) -> UnitName {
        let mut rng = rand::thread_rng();
        let letter1 = (b'A' + rng.gen_range(0..26)) as char;
        let letter2 = (b'A' + rng.gen_range(0..26)) as char;
        let numbers: u32 = rng.gen_range(0..1000);
        UnitName(format!("{letter1}{letter2}-{:03}", numbers))
    }
}
