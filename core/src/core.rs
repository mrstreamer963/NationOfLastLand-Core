use crate::defines::{MapSize, MinMax};
use crate::modules::components::{EntityType, Force, Guid, Health, MaxSpeed, Pos, Rot, UnitState, Velocity};
use crate::modules::markers::{Vehicle};

use crate::modules::exporter::export_to_json;
use crate::modules::state::State;
use crate::modules::world_state::WorldState;
use crate::modules::systems::ai_vehicle::ai_vehicle_system;
use crate::random_generator::RandomGenerator;
use hecs::World;

pub struct Core {
    world: World,
    ws: WorldState,
    r: RandomGenerator,
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
        let r = RandomGenerator {
            toxic_health: MinMax { max: 5.0, min: 1.0 },
            size: MapSize {
                width: 10,
                height: 10,
            },
        };

        let ws = WorldState::default();

        let mut c = Core { world, ws, s, r };

        c.init_world();
        c
    }

    pub fn spawn_entity(&mut self, bundle: impl hecs::Bundle + Send + Sync + 'static) -> hecs::Entity {
        let guid = Guid::new();
        let entity = self.world.spawn((guid, bundle));
        self.ws.guid_to_entity.insert(guid, entity);
        entity
    }

    pub fn create_trash(&mut self) -> Result<(), String> {
        let bundle = self.r.create_trash();
        self.spawn_entity(bundle);
        Ok(())
    }

    pub fn create_vehicle(&mut self, pos: Pos) -> Result<(), String> {
        self.spawn_entity((
            pos,
            Rot { x: 0.0, y: 0.0 },
            MaxSpeed { value: 0.1 },
            Velocity { x: 0.0, y: 0.0 },
            Health(5.0),
            Force(100.0),
            UnitState::IsWaitingTarget,
            EntityType::Vehicle,
            Vehicle {},
        ));
        Ok(())
    }

    pub fn update(&mut self, delta: f64) -> Result<(), String> {
        // Run AI system to process waiting vehicles and assign targets
        ai_vehicle_system(&mut self.world);

        // Increment time
        self.s.time += delta;

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

        self.create_trash().expect("Failed to create waste");

        self.create_trash().expect("Failed to create waste");
    }
}
