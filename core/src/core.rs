use crate::base_utils::add_floor_to_base;
use crate::descriptions::{Descriptions, load_alerts_static, load_bases_static, load_damage_types_static, load_floors_static, load_items_static, load_vehicles_static};
use crate::exporter::{export_entity_to_json, export_to_json};
use crate::internal_data::get_entity_by_guid;
use crate::modules::components::{AttachedItems, Guid, Pos};
use crate::modules::markers::Item;
use crate::modules::systems::dead_remover::do_remove_dead;
use crate::modules::systems::interaction_system::do_interaction;
use crate::modules::systems::move_system::{do_move, set_speed_by_target};
use crate::world_utils::{attach_entity, get_base_type, remove_entity};

use crate::modules::setup::{self, load_setup_static};
use crate::modules::state::State;
use crate::modules::systems::ai_vehicle::{ai_vehicle_system};
use crate::modules::systems::attack_processor::attack_process;
use crate::modules::systems::attach::attach_process;
use crate::random_generator::RandomGenerator;
use crate::spawner::{create_alert_from_description, create_base_from_description, create_item_from_description, create_vehicle_from_description};
use hecs::{Entity, World};
use std::error::Error;

const ALERTS_YAML: &str = include_str!("../../data/alerts.yml");
const BASES_YAML: &str = include_str!("../../data/bases.yml");
const DAMAGE_TYPES_YAML: &str = include_str!("../../data/damage_types.yml");
const FLOORS_YAML: &str = include_str!("../../data/floors.yml");
const ITEMS_YAML: &str = include_str!("../../data/items.yml");
const VEHICLES_YAML: &str = include_str!("../../data/vehicles.yml");
const SETUP_YAML: &str = include_str!("../../data/setup.yml");

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
        let setup = load_setup_static(SETUP_YAML).unwrap();
        let r = RandomGenerator {
            trash_probability_threshold: setup.trash_probability_threshold,
            waste_probability_threshold: setup.waste_probability_threshold,
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

    pub fn create_trash(&mut self) -> Result<Entity, String> {
        use crate::random_generator::generate_random_pos;
        let pos = generate_random_pos(&self.setup.spatial.map_size);

        create_alert_from_description(&mut self.world, &self.descriptions, "ALERT_TRASH", pos, &self.r)
    }

    pub fn create_waste(&mut self) -> Result<Entity, String> {
        use crate::random_generator::generate_random_pos;
        let pos = generate_random_pos(&self.setup.spatial.map_size);

        create_alert_from_description(&mut self.world, &self.descriptions, "ALERT_WASTE", pos, &self.r)
    }

    pub fn create_vehicle(&mut self, vehicle_key: &str, pos: Pos) -> Result<Entity, String> {
        if let Some(vehicle_data) = self.descriptions.vehicles.get(vehicle_key) {
            if self.s.reputation.0 >= vehicle_data.reputation_cost_buy {
                self.s.reputation.0 -= vehicle_data.reputation_cost_buy;
                create_vehicle_from_description(&mut self.world, &self.descriptions, vehicle_key, pos, &self.r)
            } else {
                Err(format!("Not enough reputation to create vehicle '{}'. Required: {}, available: {}", vehicle_key, vehicle_data.reputation_cost_buy, self.s.reputation.0))
            }
        } else {
            Err(format!("Vehicle '{}' not found in descriptions", vehicle_key))
        }
    }

    pub fn create_item(&mut self, item_key: &str, pos: Pos) -> Result<Entity, String> {
        create_item_from_description(&mut self.world, &self.descriptions, item_key, pos)
    }

    pub fn create_base(&mut self, base_key: &str, pos: Pos) -> Result<Entity, String> {
        create_base_from_description(&mut self.world, &self.descriptions, base_key, pos)
    }

    pub fn sell_vehicle(&mut self, vehicle_guid: Guid) -> Result<(), String> {
        // Get entity from guid
        let vehicle = get_entity_by_guid(&vehicle_guid).ok_or_else(|| format!("Vehicle with Guid {:?} not found", vehicle_guid))?;

        // Get vehicle type
        let vehicle_type = get_base_type(&self.world, vehicle)?;

        if let Some(vehicle_data) = self.descriptions.vehicles.get(&vehicle_type) {
            // Remove vehicle from world
            remove_entity(&mut self.world, vehicle)?;

            // Add reputation for selling
            self.s.reputation.0 += vehicle_data.reputation_cost_sell;

            Ok(())
        } else {
            Err(format!("Vehicle type '{}' not found in descriptions", vehicle_type))
        }
    }

    pub fn update(&mut self, delta: f64) -> Result<(), String> {

        do_remove_dead(&mut self.world, &mut self.s);

        do_interaction(&mut self.world, &self.descriptions);
        attack_process(&mut self.world);
        attach_process(&mut self.world, &self.descriptions);

        // Generate trash with probability > trash_probability_threshold
        if crate::random_generator::generate_probability() > self.r.trash_probability_threshold {
            self.create_trash()?;
        }

        // Generate waste with probability > waste_probability_threshold
        if crate::random_generator::generate_probability() > self.r.waste_probability_threshold {
            self.create_waste()?;
        }

        // Run AI system to process waiting vehicles and assign targets
        ai_vehicle_system(&mut self.world);

        set_speed_by_target(&mut self.world, &self.setup.spatial);
        do_move(&mut self.world, &self.setup.spatial);
        
        // Increment time
        self.s.time += delta;

        Ok(())
    }

    pub fn get_descriptions(&self) -> &Descriptions {
        &self.descriptions
    }

    pub fn attach_to_vehicle(&mut self, vehicle: Entity, item: Entity, slot_id: &str) -> Result<(), String> {
        // Get vehicle type
        let vehicle_type = get_base_type(&self.world, vehicle)?;

        // Check if slot exists in descriptions
        let has_slot = if let Some(vehicle_desc) = self.descriptions.vehicles.get(&vehicle_type) {
            vehicle_desc.active_slot.iter().any(|slot| slot.id == slot_id)
        } else {
            false
        };

        if has_slot {
            // Check if item is an item
            if self.world.get::<&Item>(item).is_ok() {
                // Attach item to vehicle
                attach_entity(&mut self.world, item, vehicle)?;

                if self.world.get::<&AttachedItems>(vehicle).is_err() {
                    self.world.insert_one(vehicle, AttachedItems::new()).unwrap();
                }

                let mut query = self.world.query_one::<&mut AttachedItems>(vehicle).unwrap();
                let attached_items = query.get().unwrap();
                attached_items.attach(slot_id, item);

                Ok(())
            } else {
                Err("Entity is not an item".to_string())
            }
        } else {
            Err(format!("Slot '{}' not found on vehicle '{}'", slot_id, vehicle_type))
        }
    }

    pub fn add_floor_to_base(&mut self, base: Entity, floor_type: &str) -> Result<(), String> {
        add_floor_to_base(&mut self.world, &self.descriptions, base, floor_type)
    }

    pub fn export_world(&self, is_pretty: bool) -> String {
        export_to_json(&self.world, &self.s, is_pretty)
    }

    // Export once entity
    pub fn export_entity(&self, entity: Entity, is_pretty: bool) -> String {
        export_entity_to_json(&self.world, entity, is_pretty)
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn can_create_vehicle(&self, vehicle_key: &str) -> bool {
        if let Some(vehicle_data) = self.descriptions.vehicles.get(vehicle_key) {
            self.s.reputation.0 >= vehicle_data.reputation_cost_buy
        } else {
            false
        }
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.descriptions.alerts = load_alerts_static(ALERTS_YAML)?;
        self.descriptions.bases = load_bases_static(BASES_YAML)?;
        self.descriptions.damage_types = load_damage_types_static(DAMAGE_TYPES_YAML)?;
        self.descriptions.floors = load_floors_static(FLOORS_YAML)?;
        self.descriptions.items = load_items_static(ITEMS_YAML)?.items;
        self.descriptions.vehicles = load_vehicles_static(VEHICLES_YAML)?.vehicles;
        self.descriptions.validate_attack_types()?;
        Ok(())
    }
}

impl Core {
    fn init_world(&mut self) {
        // Создание vehicle на основе данных из YAML (VEHICLE_CAR)

    let e = self.create_base("BASE_START", Pos { x: 10.0, y: 10.0 }).unwrap();
    self.add_floor_to_base(e, "FLOOR_PARK").unwrap();

    for _ in 0..1 {
        let vehicle = self.create_vehicle("VEHICLE_CAR", Pos { x: 5.0, y: 5.0 })
            .expect("Failed to create vehicle from YAML");
            // Create an item with interactions
        let item = self.create_item("ITEM_CLEANER", Pos { x: 5.0, y: 5.0 }).unwrap();
        self.attach_to_vehicle(vehicle, item, "front_left").unwrap();
    }
    }
}
