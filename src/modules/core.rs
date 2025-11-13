use crate::modules::components::Pos;
use crate::modules::entities::Vehicle;
use crate::modules::entities::Waste;
use crate::modules::exporter::export_to_json;
use hecs::World;

pub struct Core {
    world: World,
}

impl Core {
    pub fn new() -> Self {
        let world = World::new();
        Core { world }
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
