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

    // Вывод предметов
    println!("\nItems:");
    for (name, item) in &descriptions.items {
        println!("  {}:", name);
        println!("    attack_types:");
        for (attack_name, damages) in &item.attack_types {
            println!("      {}:", attack_name);
            for entry in damages {
                println!("        - type: {}, damage={}", entry.attack_type, entry.damage);
            }
        }
    }

    // Вывод транспортных средств
    println!("\nVehicles:");
    for (name, vehicle) in &descriptions.vehicles {
        println!("  {}:", name);
        println!("    max_speed: {}", vehicle.max_speed);
        println!("    max_health: {}", vehicle.max_health);
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
