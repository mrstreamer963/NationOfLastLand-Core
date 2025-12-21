use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Fraction, Pos};

fn main() {
    let mut core = Core::new(false);

    // Создание транспортного средства (vehicle)
    let vehicle = core.create_vehicle("VEHICLE_CAR", Pos::ZERO, Fraction::Neutral).unwrap();

    // Создание предмета (item)
    let item = core.create_item("ITEM_BRUSH", Pos::ZERO).unwrap();

    // Присоединение предмета к транспортному средству в слоту "front_left"
    core.attach_to_vehicle(vehicle, item, "front_left").unwrap();
    println!("First attach successful");

    // Создание второго предмета
    let item2 = core.create_item("ITEM_BRUSH", Pos { x: 1.0, y: 1.0 }).unwrap();

    // Попытка присоединить второй предмет в тот же слот
    match core.attach_to_vehicle(vehicle, item2, "front_left") {
        Ok(_) => println!("Second attach successful (unexpected)"),
        Err(e) => println!("Second attach failed as expected: {}", e),
    }

    // Вывод содержимого vehicle
    let vehicle_content = core.export_entity(vehicle, true);
    println!("Vehicle content:\n{}", vehicle_content);

    // Экспорт item для проверки Owner
    let item_content = core.export_entity(item, true);
    println!("Item content:\n{}", item_content);
}
