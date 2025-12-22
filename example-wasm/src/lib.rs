use nation_of_last_land_core::{Core, modules::components::{Pos, Fraction}};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::console;

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(Core::new(true));
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"WASM module initialized!".into());
}

// Function that uses the core crate
#[wasm_bindgen]
pub fn get_core_info() -> String {
    // This would use functionality from the core crate
    // For now, just return a placeholder
    "Core functionality integrated with WASM".to_string()
}

// Function to demonstrate async capabilities
#[wasm_bindgen]
pub async fn async_example() -> Result<String, JsValue> {
    // Simulate some async work
    let promise = js_sys::Promise::resolve(&JsValue::from_str("Async WASM result"));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result.as_string().unwrap_or_default())
}

// Function to update the world state
#[wasm_bindgen]
pub fn update_world(delta_ms: f64) {
    CORE.with(|core| {
        core.borrow_mut()
            .update(delta_ms / 1000.0)
            .unwrap_or_else(|e| {
                console::warn_1(&format!("Failed to update core: {}", e).into());
            });
    });
}

// Function to get world data for rendering
#[wasm_bindgen]
pub fn get_world_data() -> String {
    CORE.with(|core| core.borrow().export_world(false))
}

// Function to check if enough reputation to create vehicle
#[wasm_bindgen]
pub fn can_create_vehicle(vehicle_key: &str) -> bool {
    CORE.with(|core| {
        let core = core.borrow();
        core.can_create_vehicle(vehicle_key)
    })
}

// Function to create a new vehicle
#[wasm_bindgen]
pub fn create_vehicle(vehicle_key: &str, x: f32, y: f32) -> Result<String, JsValue> {
    CORE.with(|core| {
        let result = core.borrow_mut().create_vehicle(vehicle_key, Pos { x, y }, Fraction::Neutral);

        match result {
            Ok(vehicle) => {
                // TODO remove it
                let item = core.borrow_mut().create_item("ITEM_CLEANER", Pos { x: 5.0, y: 5.0 }).unwrap();
                core.borrow_mut().attach_to_vehicle(vehicle, item, "front_left").unwrap();

                Ok(format!("Vehicle '{}' created at ({:.2}, {:.2})", vehicle_key, x, y))
            },
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}

// Function to create a new base
#[wasm_bindgen]
pub fn create_base(base_key: &str, x: f32, y: f32) -> Result<String, JsValue> {
    CORE.with(|core| {
        let result = core.borrow_mut().create_base(base_key, Pos { x, y });

        match result {
            Ok(_base) => Ok(format!("Base '{}' created at ({:.2}, {:.2})", base_key, x, y)),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}

// Function to create a new unit
#[wasm_bindgen]
pub fn create_unit(unit_key: &str, x: f32, y: f32, fraction: &str) -> Result<String, JsValue> {
    use nation_of_last_land_core::modules::components::Fraction as CoreFraction;

    let core_fraction = match fraction.to_uppercase().as_str() {
        "RED" => CoreFraction::Red,
        "BLUE" => CoreFraction::Blue,
        "NEUTRAL" => CoreFraction::Neutral,
        _ => return Err(JsValue::from_str("Invalid fraction. Use RED, BLUE, or NEUTRAL")),
    };

    CORE.with(|core| {
        let result = core.borrow_mut().create_unit(unit_key, Pos { x, y }, core_fraction);

        match result {
            Ok(_unit) => Ok(format!("Unit '{}' created at ({:.2}, {:.2}) with fraction {}", unit_key, x, y, fraction)),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}

// Function to create a new floor
#[wasm_bindgen]
pub fn create_floor(floor_key: &str, x: f32, y: f32, fraction: &str) -> Result<String, JsValue> {
    use nation_of_last_land_core::modules::components::Fraction as CoreFraction;

    let core_fraction = match fraction.to_uppercase().as_str() {
        "RED" => CoreFraction::Red,
        "BLUE" => CoreFraction::Blue,
        "NEUTRAL" => CoreFraction::Neutral,
        _ => return Err(JsValue::from_str("Invalid fraction. Use RED, BLUE, or NEUTRAL")),
    };

    CORE.with(|core| {
        let result = core.borrow_mut().create_floor(floor_key, Pos { x, y }, core_fraction);

        match result {
            Ok(_floor) => Ok(format!("Floor '{}' created at ({:.2}, {:.2}) with fraction {}", floor_key, x, y, fraction)),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}

// Function to damage a unit by GUID
#[wasm_bindgen]
pub fn damage_unit(guid_str: &str, damage: f32) -> Result<String, JsValue> {
    use nation_of_last_land_core::modules::components::{Guid, Health};
    use uuid::Uuid;

    CORE.with(|core| {
        let uuid = Uuid::parse_str(guid_str).map_err(|_| JsValue::from_str("Invalid GUID format"))?;
        let guid = Guid(uuid);

        let mut core_mut = core.borrow_mut();
        let world = core_mut.get_world();

        // Find the entity and damage it in one go
        let mut found = false;
        let mut result_msg = String::new();

        for (entity, (entity_guid, mut health)) in world.query::<(&Guid, &mut Health)>().iter() {
            if *entity_guid == guid {
                health.current = (health.current - damage).max(0.0);
                result_msg = format!("Unit damaged by {:.1}, current health: {:.1}/{:.1}", damage, health.current, health.max);
                found = true;
                break;
            }
        }

        if found {
            Ok(result_msg)
        } else {
            Err(JsValue::from_str("Unit not found or has no health component"))
        }
    })
}

// Function to sell a vehicle
#[wasm_bindgen]
pub fn sell_vehicle(guid_str: &str) -> Result<String, JsValue> {
    use nation_of_last_land_core::modules::components::Guid;
    use uuid::Uuid;

    CORE.with(|core| {
        let uuid = Uuid::parse_str(guid_str).map_err(|_| JsValue::from_str("Invalid GUID format"))?;
        let guid = Guid(uuid);
        let result = core.borrow_mut().sell_vehicle(guid);

        match result {
            Ok(()) => Ok("Vehicle sold successfully".to_string()),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}
