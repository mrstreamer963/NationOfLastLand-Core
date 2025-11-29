use crate::defines::MinMax;
use crate::modules::components::{EntityType, Force, Guid, Health, MaxSpeed, Pos, Rot, Velocity};
use crate::descriptions::Descriptions;
use crate::modules::markers::{Vehicle, IsWaitingTarget};

use crate::modules::exporter::export_to_json;
use crate::modules::setup;
use crate::modules::state::State;
use crate::modules::systems::ai_vehicle::ai_vehicle_system;
use crate::random_generator::RandomGenerator;
use hecs::World;
use serde::Deserialize;
use serde_yaml;

#[derive(Deserialize)]
struct DamageTypesConfig {
    damage_types: Vec<String>,
}

const DAMAGE_TYPES_YAML: &str = include_str!("../../data/damage_types.yml");

pub struct Core {
    world: World,
    r: RandomGenerator,
    s: State,
    setup: setup::Setup,
    descriptions: Descriptions,
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
        let setup = setup::new();
        let r = RandomGenerator {
            toxic_health: MinMax { max: 5.0, min: 1.0 },
        };
        let descriptions = Descriptions::default();

        let mut c = Core { world, s, r, setup, descriptions };

        c.load().expect("Failed to load damage types");

        c.init_world();
        c
    }

    pub fn spawn_entity(&mut self, bundle: impl hecs::Bundle + Send + Sync + 'static) -> hecs::Entity {
        let guid = Guid::new();
        let entity = self.world.spawn(bundle);
        self.world.insert_one(entity, guid).unwrap();
        entity
    }

    pub fn create_trash(&mut self) -> Result<(), String> {
        let bundle = self.r.get_bundle_trash(&self.setup.spatial.map_size);
        self.spawn_entity(bundle);
        Ok(())
    }

    pub fn create_vehicle(&mut self, pos: Pos) -> Result<(), String> {
        self.spawn_entity((
            pos,
            Rot { x: 0.0, y: 0.0 },
            MaxSpeed { value: 0.1 },
            Velocity { x: 0.0, y: 0.0 },
            Health { current: 5.0, max: 5.0 },
            Force(100.0),
            IsWaitingTarget {},
            EntityType::Vehicle,
            Vehicle {},
        ));
        Ok(())
    }

    pub fn update(&mut self, delta: f64) -> Result<(), String> {
        // Run AI system to process waiting vehicles and assign targets
        ai_vehicle_system(&mut self.world, &self.setup.spatial);

        // Increment time
        self.s.time += delta;

        Ok(())
    }

    pub fn get_descriptions(&self) -> &Descriptions {
        &self.descriptions
    }

    pub fn get_descriptions_mut(&mut self) -> &mut Descriptions {
        &mut self.descriptions
    }

    pub fn export_world(&self) -> String {
        export_to_json(&self.world, &self.s)
    }

    fn load(&mut self) -> Result<(), serde_yaml::Error> {
        let config: DamageTypesConfig = serde_yaml::from_str(DAMAGE_TYPES_YAML)?;
        self.descriptions.damage_types = config.damage_types;
        Ok(())
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
