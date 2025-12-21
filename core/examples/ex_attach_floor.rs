use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Owner, Pos};

fn main() {
    let mut core = Core::new(false);

    // Создание базы
    let base = core.create_base("BASE_MAIN", Pos::ZERO).unwrap();

    // Присоединение первого этажа к базе
    core.add_floor_to_base(base, "FLOOR_PARK").unwrap();
    core.update(0.0).unwrap(); // Process attach events
    println!("Attached FLOOR_PARK successfully");

    // Попытка присоединить второй этаж (должен быть отказ, так как max_floors = 1)
    match core.add_floor_to_base(base, "FLOOR_ENTERANCE") {
        Ok(_) => {
            core.update(0.0).unwrap(); // Process attach events
            println!("Attached FLOOR_ENTERANCE successfully");
        }
        Err(e) => println!("Failed to attach FLOOR_ENTERANCE: {}", e),
    }

    // Вывод содержимого базы
    let base_content = core.export_entity(base, true);
    println!("Base content:\n{}", base_content);

    // Получение количества этажей
    let floor_count = core.get_world().query::<&Owner>()
        .iter()
        .filter(|(_, owner)| owner.e == base)
        .count();

    println!("Floor count: {}", floor_count);
}
