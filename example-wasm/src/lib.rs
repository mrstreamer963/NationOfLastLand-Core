use nation_of_last_land_core::{Core, modules::components::Pos};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::console;

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(Core::new());
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
        let result = core.borrow_mut().create_vehicle(vehicle_key, Pos { x, y });

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
