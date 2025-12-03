use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::Pos;

fn main() {
    let mut core = Core::new();

    // Создание транспортного средства (vehicle)
    let vehicle = core.create_vehicle_from_yaml("VEHICLE_CAR", Pos { x: 0.0, y: 0.0 }).unwrap();

    // Создание предмета (item)
    let item = core.create_item_from_yaml("ITEM_CLEANER", Pos { x: 0.0, y: 0.0 }).unwrap();

    // Присоединение предмета к транспортному средству в слоту "front_left"
    core.attach(vehicle, item, "front_left").unwrap();

    // Вывод содержимого vehicle
    let vehicle_content = core.export_entity(vehicle, true);
    println!("Vehicle content:\n{}", vehicle_content);

    // Экспорт item для проверки Owner
    let item_content = core.export_entity(item, true);
    println!("Item content:\n{}", item_content);
}
