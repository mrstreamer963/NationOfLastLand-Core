use crate::defines::MinMax;
use crate::descriptions::{Descriptions, load_damage_types_static, load_items_static, load_vehicles_static};
use crate::modules::components::{AttachedItems, BaseType, EntityType, Force, Guid, Health, MaxSpeed, Owner, Pos, Rot, Velocity, WeaponMode, WeaponType};
use crate::modules::markers::{IsWaitingTarget, Vehicle, Item};

use crate::modules::exporter::{export_to_json, export_entity_to_json};
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
        if let Some(vehicle_data_ref) = self.descriptions.vehicles.get(vehicle_key) {
            let vehicle_data = vehicle_data_ref.clone();
            let e = self.spawn_entity((
                BaseType(vehicle_key.to_string()),
                pos,
                Rot { x: 0.0, y: 0.0 },
                MaxSpeed(vehicle_data.max_speed),
                Velocity { x: 0.0, y: 0.0 },
                Health { current: vehicle_data.max_health, max: vehicle_data.max_health },
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

    pub fn create_item_from_yaml(&mut self, item_key: &str, _pos: Pos) -> Result<Entity, String> {
        if let Some(item_data) = self.descriptions.items.get(item_key) {
            let mut modes = Vec::new();
            for interaction in &item_data.interactions {
                for (dmg_type, dmg_value) in &interaction.action {
                    modes.push(WeaponMode {
                        damage_type: dmg_type.clone(),
                        damage: *dmg_value as i32,
                        range: 1.0,
                    });
                }
            }
            let e = self.spawn_entity((
                BaseType(item_key.to_string()),
                EntityType::Item,
                Item {},
            ));
            if !modes.is_empty() {
                let weapon_type = WeaponType { modes };
                self.world.insert_one(e, weapon_type).unwrap();
            }
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

    pub fn attach(&mut self, vehicle: Entity, item: Entity, slot_id: &str) -> Result<(), String> {
        // Get vehicle type
        let vehicle_type = {
            if let Ok(mut query) = self.world.query_one::<&BaseType>(vehicle) {
                if let Some(base_type) = query.get() {
                    base_type.0.clone()
                } else {
                    return Err("Vehicle has no BaseType component".to_string());
                }
            } else {
                return Err("Vehicle not found".to_string());
            }
        };

        // Check if slot exists in descriptions
        let has_slot = if let Some(vehicle_desc) = self.descriptions.vehicles.get(&vehicle_type) {
            vehicle_desc.active_slot.iter().any(|slot| slot.id == slot_id)
        } else {
            false
        };

        if has_slot {
            // Check if item is an item
            if self.world.get::<&Item>(item).is_ok() {
                // Insert Owner component to item
                self.world.insert_one(item, Owner(vehicle)).map_err(|_| "Failed to insert Owner component".to_string())?;

                // Add reference to item in vehicle's AttachedItems
                self.world.insert_one(vehicle, AttachedItems::new()).map_err(|_| "Failed to insert AttachedItems component".to_string())?;
                if let Ok(mut query) = self.world.query_one::<&mut AttachedItems>(vehicle) {
                    let attached_items = query.get().unwrap();
                    attached_items.attach(slot_id, item);
                } else {
                    return Err("Failed to query AttachedItems on vehicle".to_string());
                }

                Ok(())
            } else {
                Err("Entity is not an item".to_string())
            }
        } else {
            Err(format!("Slot '{}' not found on vehicle '{}'", slot_id, vehicle_type))
        }
    }

    pub fn export_world(&self, is_pretty: bool) -> String {
        export_to_json(&self.world, &self.s, is_pretty)
    }

    pub fn export_entity(&self, entity: Entity, is_pretty: bool) -> String {
        export_entity_to_json(&self.world, entity, is_pretty)
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
