use nation_of_last_land_core::Core;

fn main() {
    // Создание Core и автоматический вызов load() в конструкторе
    let core = Core::new(false);

    // Получение описаний, загруженных из YAML
    let descriptions = core.get_descriptions();

    // Вывод типов повреждений
    println!("Damage Types:");
    for (index, damage_type) in descriptions.damage_types.iter().enumerate() {
        println!("  {}. {}", index + 1, damage_type);
    }

    // Вывод алертов
    println!("\nAlerts:");
    for (name, alert) in &descriptions.alerts {
        println!("  {}:", name);
        println!("    reputation_cost_destroy: {}", alert.reputation_cost_destroy);
        if let Some(interactions) = &alert.interactions {
            println!("    interactions:");
            for interaction in interactions {
                println!("      {}:", interaction.1.name);
                for (effect, value) in &interaction.1.effects {
                    println!("        {}: {}", effect, value);
                }
            }
        }
    }

    // Вывод предметов
    println!("\nItems:");
    for (name, item) in &descriptions.items {
        println!("  {}:", name);
        if let Some(interactions) = &item.interactions {
            println!("    interactions:");
            for interaction in interactions {
                println!("      {}:", interaction.1.name);
                if let Some(range) = interaction.1.range {
                    println!("        range: {}", range);
                }
                for (damage_type, damage_value) in &interaction.1.effects {
                    println!("        - {}: {}", damage_type, damage_value);
                }
            }
        }
        if let Some(takeable) = item.takeable {
            println!("    takeable: {}", takeable);
        }
    }



    // Вывод транспортных средств (из units)
    println!("\nVehicles (from units):");
    for (name, unit) in &descriptions.units {
        if unit.max_speed.is_some() {  // Это транспортное средство
            println!("  {}:", name);
            if let Some(max_speed) = &unit.max_speed {
                println!("    max_speed: {:?}", max_speed);
            }
            if let Some(max_health) = &unit.max_health {
                println!("    max_health: {:.2} - {:.2}", max_health.min, max_health.max);
            }
            if let Some(slots_type) = &unit.slots_type {
                println!("    slots_type: {}", slots_type);
                // Display slots from slots_types if available
                if let Some(slots) = descriptions.slots_types.get(slots_type) {
                    println!("    slots:");
                    for slot in slots {
                        println!("      - id: {}", slot.id);
                        println!("        slot_tags: {:?}", slot.slot_tags);
                        println!("        mount_point: {}", slot.mount_point);
                    }
                }
            }
            if let Some(buy_cost) = unit.reputation_cost_buy {
                println!("    reputation_cost_buy: {}", buy_cost);
            }
            if let Some(sell_cost) = unit.reputation_cost_sell {
                println!("    reputation_cost_sell: {}", sell_cost);
            }
            if let Some(destroy_cost) = unit.reputation_cost_destroy {
                println!("    reputation_cost_destroy: {}", destroy_cost);
            }
        }
    }
}
