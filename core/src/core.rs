use crate::defines::{MapSize, MinMax, Point};
use crate::modules::components::{IsMoving, IsWaitingTarget, MaxSpeed, Pos, Rot, Target};
use crate::modules::entities::Vehicle;
use crate::modules::exporter::export_to_json;
use crate::modules::state::State;
use crate::modules::systems::ai_vehicle::ai_vehicle_system;
use crate::random_generator::RandomGenerator;
use hecs::World;

pub struct Core {
    world: World,
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
            toxic_power: MinMax { max: 1.0, min: 1.0 },
            size: MapSize {
                width: 10,
                height: 10,
            },
        };

        let mut c = Core { world, s, r };

        c.init_world();
        c
    }

    pub fn create_waste(&mut self) -> Result<(), String> {
        self.r.create_waste(&mut self.world);
        Ok(())
    }

    pub fn create_vehicle(&mut self, pos: Pos) -> Result<(), String> {
        self.world.spawn((
            pos,
            Rot { x: 0.0, y: 0.0 },
            MaxSpeed { value: 1.0 },
            IsWaitingTarget {},
            Vehicle {},
        ));
        Ok(())
    }

    pub fn update(&mut self, delta: f64) -> Result<(), String> {
        // Run AI system to find nearest waste targets for all vehicles
        let vehicle_targets = ai_vehicle_system(&self.world);

        // Use the vehicle targets to update vehicle behavior
        for (entity, nearest_waste) in vehicle_targets {
            if let Some(pos) = nearest_waste {
                // Assign target
                let target = Target {
                    value: Point { x: pos.x, y: pos.y },
                };
                self.world.insert_one(entity, target).unwrap();

                // Remove waiting state
                self.world.remove_one::<IsWaitingTarget>(entity).unwrap();

                // Add moving state
                self.world.insert_one(entity, IsMoving {}).unwrap();
            }
        }

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

        self.create_waste().expect("Failed to create waste");

        self.create_waste().expect("Failed to create waste");
    }
}
