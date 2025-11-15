use hecs::World;
use rand::Rng;

use crate::{
    defines::{MapSize, MaxMin},
    modules::{
        components::{Pos, ToxicPower},
        entities::Waste,
    },
};

pub struct RandomGenerator {
    size: MapSize,
    toxicPower: MaxMin,
}

pub fn generate_between(range: &MaxMin) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range.min..=range.max)
}

fn generate_random_pos(map_size: &MapSize) -> Pos {
    let mut rng = rand::thread_rng();
    Pos {
        x: rng.gen_range(0.0..=map_size.width as f32),
        y: rng.gen_range(0.0..=map_size.height as f32),
    }
}

impl RandomGenerator {
    pub fn create_waste(&self, world: &mut World) {
        let pos = generate_random_pos(&self.size);
        let level = 5.0_f32.min(self.toxicPower.max);
        world.spawn((pos, ToxicPower { level }, Waste {}));
    }
}
