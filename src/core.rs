use crate::modules::components::Pos;
use crate::modules::entities::{Vehicle, Waste};
use crate::modules::exporter::export_to_json;
use crate::state::State;
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
        Core { world, s }
    }

    pub fn create_waste(&mut self, pos: Pos) -> Result<(), String> {
        self.world.spawn((pos, Waste {}));
        Ok(())
    }

    pub fn create_vehicle(&mut self, pos: Pos) -> Result<(), String> {
        self.world.spawn((pos, Vehicle {}));
        Ok(())
    }

    pub fn update(&mut self, _delta: f64) -> Result<(), String> {
        Ok(())
    }

    pub fn export_world(&self) -> String {
        export_to_json(&self.world)
    }
}
