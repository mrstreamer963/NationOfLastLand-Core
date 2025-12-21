use nation_of_last_land_core::descriptions::load_units_static;


fn main() {
    const UNITS_YAML: &str = include_str!("../../data_v2/units.yml");

    let units = match load_units_static(UNITS_YAML) {
        Ok(units) => units,
        Err(e) => {
            println!("Failed to parse units.yml: {}", e);
            return;
        }
    };

    println!("Vehicles (units with max_speed) parsed successfully:");
    for (name, unit) in &units {
        if let Some(max_speed) = &unit.max_speed {
            println!("  Vehicle: {}", name);
            println!("    Max Speed: {:.2} - {:.2}", max_speed.min, max_speed.max);
            if let Some(max_health) = &unit.max_health {
                println!("    Max Health: {:.2} - {:.2}", max_health.min, max_health.max);
            }
            if let Some(slots_type) = &unit.slots_type {
                println!("    Slots Type: {}", slots_type);
            }
            if let Some(buy_cost) = unit.reputation_cost_buy {
                println!("    Reputation Buy: {:.2}", buy_cost);
            }
            if let Some(sell_cost) = unit.reputation_cost_sell {
                println!("    Reputation Sell: {:.2}", sell_cost);
            }
            if let Some(destroy_cost) = unit.reputation_cost_destroy {
                println!("    Reputation Destroy: {:.2}", destroy_cost);
            }
            println!();
        }
    }

    println!("Validation complete.");
}
