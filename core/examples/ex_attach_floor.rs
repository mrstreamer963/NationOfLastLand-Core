use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Floors, Pos};

fn main() {
    let mut core = Core::new();

    // Создание базы
    let base = core.create_base("BASE_MAIN", Pos { x: 0.0, y: 0.0 }).unwrap();

    // Присоединение первого этажа к базе
    core.attach_floor_to_base(base, "FLOOR_PARK").unwrap();
    println!("Attached FLOOR_PARK successfully");

    // Попытка присоединить второй этаж (должен быть отказ, так как max_floors = 1)
    match core.attach_floor_to_base(base, "FLOOR_ENTERANCE") {
        Ok(_) => println!("Attached FLOOR_ENTERANCE successfully"),
        Err(e) => println!("Failed to attach FLOOR_ENTERANCE: {}", e),
    }

    // Вывод содержимого базы
    let base_content = core.export_entity(base, true);
    println!("Base content:\n{}", base_content);

    // Получение floor_entity
    let floor_entity_opt = if let Ok(floors) = core.get_world().get::<&Floors>(base) {
        floors.0.first().copied()
    } else {
        None
    };

    // Вывод содержимого этажа
    if let Some(floor_entity) = floor_entity_opt {
        let floor_content = core.export_entity(floor_entity, true);
        println!("Floor content:\n{}", floor_content);
    }
}
