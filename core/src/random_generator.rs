use hecs::World;
use rand::Rng;

use crate::modules::{
    components::{Pos, ToxicPower},
    entities::Waste,
};

pub struct RandomGenerator {}

fn generate_random_pos() -> Pos {
    let mut rng = rand::thread_rng();
    Pos {
        x: rng.gen_range(0.0..=10.0),
        y: rng.gen_range(0.0..=10.0),
    }
}

impl RandomGenerator {
    pub fn create_waste(&self, world: &mut World) {
        let pos = generate_random_pos();
        world.spawn((pos, ToxicPower { level: 5.0 }, Waste {}));
    }
}
