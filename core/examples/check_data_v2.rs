use nation_of_last_land_core::descriptions::load_items_static;
use std::fs;

fn main() {
    // Load data_v2/items.yml
    let items_yaml = fs::read_to_string("data_v2/items.yml").expect("Failed to read items.yml");
    let items = match load_items_static(&items_yaml) {
        Ok(items) => items,
        Err(e) => {
            println!("Failed to parse items.yml: {}", e);
            return;
        }
    };

    println!("Items parsed successfully:");
    for (name, item) in &items {
        println!("  {}: {:?}", name, item);
    }

    println!("\nValidation complete.");
}
