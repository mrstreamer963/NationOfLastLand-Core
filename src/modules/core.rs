use crate::modules::components::Pos;
use crate::modules::entities::Vehicle;
use crate::modules::entities::Waste;
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
}
