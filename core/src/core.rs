use crate::modules::components::{Pos, Rot};
use crate::modules::entities::{Vehicle, Waste};
use crate::modules::exporter::export_to_json;
use crate::modules::state::State;
use hecs::World;

pub struct Core {
    world: World,
    s: State,
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Core {
    pub fn new() -> Self {
        let world = World::new();
        let s = State::new();
        let mut c = Core { world, s };

        c.init_world();
        c
    }

    pub fn create_waste(&mut self, pos: Pos) -> Result<(), String> {
        self.world.spawn((pos, Waste {}));
        Ok(())
    }

    pub fn create_vehicle(&mut self, pos: Pos) -> Result<(), String> {
        self.world.spawn((pos, Rot { x: 0.0, y: 0.0 }, Vehicle {}));
        Ok(())
    }

    pub fn update(&mut self, _delta: f64) -> Result<(), String> {
        Ok(())
    }

    pub fn export_world(&self) -> String {
        export_to_json(&self.world, &self.s)
    }
}

impl Core {
    fn init_world(&mut self) {
        self.create_vehicle(Pos { x: 1.0, y: 1.0 })
            .expect("Failed to create vehicle");

        self.create_waste(Pos { x: 5.0, y: 5.0 })
            .expect("Failed to create waste");

        self.create_waste(Pos { x: 10.0, y: 10.0 })
            .expect("Failed to create waste");
    }
}
