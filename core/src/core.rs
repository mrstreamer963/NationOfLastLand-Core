use crate::base_utils::add_floor_to_base;
use crate::descriptions::{Descriptions, load_alerts_static, load_bases_static, load_damage_types_static, load_slot_tags_static, load_slots_types_static, load_items_static, load_units_static};
use crate::exporter::{export_entity_to_json, export_to_json};
use crate::internal_data::get_entity_by_guid;
use crate::modules::components::{AttachedItems, Fraction, Guid, Pos};
use crate::modules::markers::Item;
use crate::modules::systems::ai_unit::ai_unit_system;
use crate::modules::systems::dead_remover::do_remove_dead;
use crate::modules::systems::interaction_system::do_interaction;
use crate::modules::systems::move_system::{do_move, set_speed_by_target};
use crate::world_utils::{attach_entity, get_base_type, remove_entity};

use crate::modules::setup::{self, load_setup_static};
use crate::modules::state::State;
use crate::modules::systems::attack_processor::attack_process;
use crate::modules::systems::attach::attach_process;
use crate::random_generator::RandomGenerator;
use crate::spawner::{create_alert_from_description, create_base_from_description, create_item_from_description, create_unit_from_description, create_vehicle_from_description, create_floor_from_description, fill_unit_inventory};
use hecs::{Entity, World};
use std::error::Error;

const SLOTS_TAGS_YAML: &str = include_str!("../../data_v2/slots_tags.yml");
const SLOTS_TYPES_YAML: &str = include_str!("../../data_v2/slots_types.yml");
const ITEMS_YAML: &str = include_str!("../../data_v2/items.yml");
const UNITS_YAML: &str = include_str!("../../data_v2/units.yml");
const FLOORS_YAML: &str = include_str!("../../data_v2/floors.yml");

const ALERTS_YAML: &str = include_str!("../../data/alerts.yml");
const BASES_YAML: &str = include_str!("../../data/bases.yml");
const DAMAGE_TYPES_YAML: &str = include_str!("../../data/damage_types.yml");
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
        Self::new(true)
    }
}

impl Core {
    pub fn new(is_need_init: bool) -> Self {
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

        if is_need_init {
            c.init_world();
        }
        
        c
    }

    pub fn create_trash(&mut self) -> Result<Entity, String> {
        use crate::random_generator::generate_random_pos;
        let pos = generate_random_pos(&self.setup.spatial.map_size);

        let e = self.create_unit("UNIT_WASTE", pos, Fraction::Red).unwrap();
        Ok(e)
        // create_alert_from_description(&mut self.world, &self.descriptions, "ALERT_TRASH", pos, &self.r)
    }

    pub fn create_waste(&mut self) -> Result<Entity, String> {
        use crate::random_generator::generate_random_pos;
        let pos = generate_random_pos(&self.setup.spatial.map_size);

        let e = self.create_unit("UNIT_WASTE", pos, Fraction::Red).unwrap();
        // create_alert_from_description(&mut self.world, &self.descriptions, "ALERT_WASTE", pos, &self.r)
        Ok(e)
    }

    pub fn create_vehicle(&mut self, vehicle_key: &str, pos: Pos, faction: Fraction) -> Result<Entity, String> {
        let vehicle_data = self.descriptions.units.get(vehicle_key)
            .ok_or(format!("Vehicle '{}' not found in descriptions", vehicle_key))?;

        let cost = vehicle_data.reputation_cost_buy.ok_or(format!("Vehicle '{}' has no buy cost", vehicle_key))?;
        if self.s.reputation.0 >= cost {
            self.s.reputation.0 -= cost;
            let e = create_vehicle_from_description(&mut self.world, &self.descriptions, vehicle_key, pos, faction, &self.r).unwrap();
            fill_unit_inventory(&mut self.world, &self.descriptions, e, &self.r)?;
            Ok(e)
        } else {
            Err(format!("Not enough reputation to create vehicle '{}'. Required: {}, available: {}", vehicle_key, cost, self.s.reputation.0))
        }
    }

    pub fn create_floor(&mut self, floor_key: &str, pos: Pos, faction: Fraction) -> Result<Entity, String> {
        let floor_data = self.descriptions.units.get(floor_key)
            .ok_or(format!("Floor '{}' not found in descriptions", floor_key))?;

        let cost = floor_data.reputation_cost_buy.ok_or(format!("Floor '{}' has no buy cost", floor_key))?;
        if self.s.reputation.0 >= cost {
            self.s.reputation.0 -= cost;
            let e = create_floor_from_description(&mut self.world, &self.descriptions, floor_key).unwrap();
            // Add position and faction
            self.world.insert(e, (pos, faction)).unwrap();
            Ok(e)
        } else {
            Err(format!("Not enough reputation to create floor '{}'. Required: {}, available: {}", floor_key, cost, self.s.reputation.0))
        }
    }

    pub fn create_item(&mut self, item_key: &str, pos: Pos) -> Result<Entity, String> {
        create_item_from_description(&mut self.world, &self.descriptions, item_key, pos)
    }

    pub fn create_base(&mut self, base_key: &str, pos: Pos) -> Result<Entity, String> {
        create_base_from_description(&mut self.world, &self.descriptions, base_key, pos)
    }

    pub fn create_unit(&mut self, unit_key: &str, pos: Pos, faction: Fraction) -> Result<Entity, String> {
        create_unit_from_description(&mut self.world, &self.descriptions, unit_key, pos, faction, &self.r)
    }

    pub fn sell_vehicle(&mut self, vehicle_guid: Guid) -> Result<(), String> {
        // Get entity from guid
        let vehicle = get_entity_by_guid(&vehicle_guid).ok_or_else(|| format!("Vehicle with Guid {:?} not found", vehicle_guid))?;

        // Get vehicle type
        let vehicle_type = get_base_type(&self.world, vehicle)?;

        if let Some(vehicle_data) = self.descriptions.units.get(&vehicle_type) {
            // Remove vehicle from world
            remove_entity(&mut self.world, vehicle)?;

            // Add reputation for selling
            if let Some(sell_cost) = vehicle_data.reputation_cost_sell {
                self.s.reputation.0 += sell_cost;
            }

            Ok(())
        } else {
            Err(format!("Vehicle type '{}' not found in descriptions", vehicle_type))
        }
    }

    pub fn update(&mut self, delta: f64) -> Result<(), String> {

        do_remove_dead(&mut self.world, &mut self.s, &self.descriptions);

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
        // ai_vehicle_system(&mut self.world);
        ai_unit_system(&mut self.world, &self.descriptions);

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

        // Get item type
        let item_type = get_base_type(&self.world, item)?;

        // Get vehicle description
        let vehicle_desc = self.descriptions.units.get(&vehicle_type)
            .ok_or(format!("Vehicle '{}' not found in descriptions", vehicle_type))?;

        let slots_type = vehicle_desc.slots_type.as_ref().ok_or(format!("Vehicle '{}' has no slots_type", vehicle_type))?;

        // Get item description
        let item_desc = self.descriptions.items.get(&item_type)
            .ok_or(format!("Item '{}' not found in descriptions", item_type))?;

        // Get item tags
        let item_tags = item_desc.tags.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);

        // Get slots for vehicle
        let slots = self.descriptions.slots_types.get(slots_type)
            .ok_or(format!("Slots type '{}' not found", slots_type))?;

        // Find the slot
        let slot = slots.iter().find(|s| s.id == slot_id)
            .ok_or(format!("Slot '{}' not found on vehicle '{}'", slot_id, vehicle_type))?;

        // Check if tags match (at least one common tag)
        let has_common_tag = item_tags.iter().any(|tag| slot.slot_tags.contains(tag));
        if !has_common_tag {
            return Err(format!("Item '{}' tags {:?} do not match slot '{}' tags {:?}", item_type, item_tags, slot_id, slot.slot_tags));
        }

        // Check if item is an item
        if self.world.get::<&Item>(item).is_ok() {
            // Check if slot is already occupied
            if let Ok(mut query) = self.world.query_one::<&AttachedItems>(vehicle) {
                if let Some(attached_items) = query.get() {
                    if attached_items.get(slot_id).is_some() {
                        return Err(format!("Slot '{}' is already occupied", slot_id));
                    }
                }
            }

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
    }

    pub fn add_floor_to_base(&mut self, base: Entity, floor_type: &str) -> Result<(), String> {
        add_floor_to_base(&mut self.world, &self.descriptions, base, floor_type)?;
        Ok(())
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
        if let Some(vehicle_data) = self.descriptions.units.get(vehicle_key) {
            if let Some(cost) = vehicle_data.reputation_cost_buy {
                self.s.reputation.0 >= cost
            } else {
                false
            }
        } else {
            false
        }
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.descriptions.slot_tags = load_slot_tags_static(SLOTS_TAGS_YAML)?;
        self.descriptions.slots_types = load_slots_types_static(SLOTS_TYPES_YAML)?;
        self.descriptions.damage_types = load_damage_types_static(DAMAGE_TYPES_YAML)?;
        self.descriptions.items = load_items_static(ITEMS_YAML)?;

        self.descriptions.alerts = load_alerts_static(ALERTS_YAML)?;
        self.descriptions.bases = load_bases_static(BASES_YAML)?;

        // Load units
        let mut units = load_units_static(UNITS_YAML)?;
        // Load floors as units and merge
        let floors = load_units_static(FLOORS_YAML)?;
        units.extend(floors);
        self.descriptions.units = units;

        self.descriptions.validate_slot_tags()?;
        self.descriptions.validate_attack_types()?;
        Ok(())
    }

}

impl Core {
    fn init_world(&mut self) {
        // Создание vehicle на основе данных из YAML (VEHICLE_CAR)

    // let e = self.create_base("BASE_START", Pos { x: 10.0, y: 10.0 }).unwrap();
    // self.add_floor_to_base(e, "FLOOR_PARK").unwrap();

    // self.create_unit("UNIT_TRASH", Pos { x: 1.0, y: 1.0 }, Fraction::Red).unwrap();
    // self.create_unit("UNIT_WASTE", Pos { x: 4.0, y: 4.0 }, Fraction::Red).unwrap();

    self.create_floor("FLOOR_PARK", Pos { x: 1.0, y: 1.0 }, Fraction::Neutral).unwrap();

    for _ in 0..1 {
        let _vehicle = self.create_vehicle("VEHICLE_CAR", Pos { x: 5.0, y: 5.0 }, Fraction::Neutral)
            .expect("Failed to create vehicle from YAML");
            // Create an item with interactions
            // let item = self.create_item("ITEM_CLEANER", Pos { x: 5.0, y: 5.0 }).unwrap();
            // self.attach_to_vehicle(vehicle, item, "front_left").unwrap();
        }
    }
}
