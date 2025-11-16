use hecs::World;
use rand::Rng;

use crate::{
    defines::{MapSize, MinMax},
    modules::{
        components::{Pos, ToxicPower},
        entities::Waste,
    },
};

pub struct RandomGenerator {
    pub size: MapSize,
    pub toxic_power: MinMax,
}

pub fn generate_between(range: &MinMax) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range.min..=range.max)
}

fn generate_random_pos(map_size: &MapSize) -> Pos {
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
    pub fn create_waste(&self, world: &mut World) {
        let pos = generate_random_pos(&self.size);
        let level = 5.0_f32.min(self.toxic_power.max);
        world.spawn((pos, ToxicPower { level }, Waste {}));
    }
}
