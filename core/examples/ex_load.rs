use nation_of_last_land_core::Core;

fn main() {
    // Создание Core и автоматический вызов load() в конструкторе
    let core = Core::new();

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
                println!("      {}:", interaction.name);
                for (effect, value) in &interaction.effects {
                    println!("        {}: {}", effect, value);
                }
            }
        }
    }

    // Вывод предметов
    println!("\nItems:");
    for (name, item) in &descriptions.items {
        println!("  {}:", name);
        println!("    interactions:");
        for interaction in &item.interactions {
            println!("      {}:", interaction.name);
            for (action_type, action_value) in &interaction.action {
                println!("        - {}: {}", action_type, action_value);
            }
        }
        if let Some(throwable) = item.throwable {
            println!("    throwable: {}", throwable);
        }
        if let Some(takeable) = item.takeable {
            println!("    takeable: {}", takeable);
        }
    }

    // Вывод транспортных средств
    println!("\nVehicles:");
    for (name, vehicle) in &descriptions.vehicles {
        println!("  {}:", name);
        println!("    max_speed: {:?}", vehicle.max_speed);
        println!("    max_health: {:?}", vehicle.max_health);
        println!("    reputation_cost: {}", vehicle.reputation_cost);
        println!("    reputation_cost_destroy: {}", vehicle.reputation_cost_destroy);
        if !vehicle.active_slot.is_empty() {
            println!("    active_slot:");
            for slot in &vehicle.active_slot {
                println!("      - id: {}", slot.id);
                println!("        slot_type: {}", slot.slot_type);
                println!("        mount_point: {}", slot.mount_point);
            }
        }
    }
}
