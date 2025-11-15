use wasm_bindgen::prelude::*;
use web_sys::console;
use console_error_panic_hook;
use nation_of_last_land_core::Core;

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

// Function to get world data for rendering
#[wasm_bindgen]
pub fn get_world_data() -> String {
    let core = Core::new();
    core.export_world()
}
