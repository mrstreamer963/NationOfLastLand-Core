use crate::defines::MinMax;
use crate::descriptions::{Descriptions, load_damage_types_static, load_items_static, load_vehicles_static};
use crate::modules::components::{EntityType, Force, Guid, Pos, Rot, Velocity, WeaponType, WeaponMode};
use crate::modules::markers::{IsWaitingTarget, Vehicle, Item};

use crate::modules::exporter::export_to_json;
use crate::modules::setup;
use crate::modules::state::State;
use crate::modules::systems::ai_vehicle::ai_vehicle_system;
use crate::random_generator::RandomGenerator;
use hecs::{Entity, World};
use std::error::Error;

const DAMAGE_TYPES_YAML: &str = include_str!("../../data/damage_types.yml");
const ITEMS_YAML: &str = include_str!("../../data/items.yml");
const VEHICLES_YAML: &str = include_str!("../../data/vehicles.yml");

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

        let mut c = Core {
            world,
            s,
            r,
            setup,
            descriptions,
        };

        c.load().expect("Failed to load damage types");

        c.init_world();
        c
    }

    pub fn spawn_entity(
        &mut self,
        bundle: impl hecs::Bundle + Send + Sync + 'static,
    ) -> hecs::Entity {
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

    pub fn create_vehicle_from_yaml(&mut self, vehicle_key: &str, pos: Pos) -> Result<Entity, String> {
        if let Some(vehicle_data) = self.descriptions.vehicles.get(vehicle_key) {
            let e = self.spawn_entity((
                pos,
                Rot { x: 0.0, y: 0.0 },
                vehicle_data.max_speed,
                Velocity { x: 0.0, y: 0.0 },
                vehicle_data.health,
                Force(100.0),
                IsWaitingTarget {},
                EntityType::Vehicle,
                Vehicle {},
            ));
            Ok(e)
        } else {
            Err(format!("Vehicle '{}' not found in descriptions", vehicle_key))
        }
    }

    pub fn create_item_from_yaml(&mut self, item_key: &str, pos: Pos) -> Result<Entity, String> {
        if let Some(item_data) = self.descriptions.items.get(item_key) {
            let mut modes = Vec::new();
            for attack_type_list in item_data.attack_types.values() {
                for attack in attack_type_list {
                    modes.push(WeaponMode {
                        damage_type: attack.attack_type.clone(),
                        damage: attack.damage as i32,
                        range: 1.0,
                    });
                }
            }
            let weapon_type = WeaponType { modes };
            let e =self.spawn_entity((
                pos,
                Rot { x: 0.0, y: 0.0 },
                weapon_type,
                Velocity { x: 0.0, y: 0.0 },
                EntityType::Item,
                Item {},
            ));
            Ok(e)
        } else {
            Err(format!("Item '{}' not found in descriptions", item_key))
        }
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

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.descriptions.damage_types = load_damage_types_static(DAMAGE_TYPES_YAML)?;
        self.descriptions.items = load_items_static(ITEMS_YAML)?.items;
        self.descriptions.vehicles = load_vehicles_static(VEHICLES_YAML)?.vehicles;
        self.descriptions.validate_attack_types()?;
        Ok(())
    }
}

impl Core {
    fn init_world(&mut self) {
        // Создание vehicle на основе данных из YAML (VEHICLE_CAR)
        self.create_vehicle_from_yaml("VEHICLE_CAR", Pos { x: 1.0, y: 1.0 })
            .expect("Failed to create vehicle from YAML");

        self.create_trash().expect("Failed to create waste");

        self.create_trash().expect("Failed to create waste");
    }
}
