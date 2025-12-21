use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Fraction, Pos};

fn main() {
    let mut core = Core::new(false);

    // Создаем юнит с инвентарем
    let unit_entity = core.create_vehicle("VEHICLE_CAR", Pos { x: 10.0, y: 10.0 }, Fraction::Neutral).unwrap();
    println!("Created unit entity: {:?}", unit_entity);

    // Проверяем инвентарь до update
    let world = core.get_world();
    if let Ok(inventory) = world.get::<&nation_of_last_land_core::modules::components::Inventory>(unit_entity) {
        println!("Inventory items before update: {:?}", inventory.0.len());
        for (i, item) in inventory.0.iter().enumerate() {
            println!("  Item {}: {:?}", i, item);
        }
    } else {
        println!("No inventory component");
    }

    core.update(0.01).unwrap();

    // Проверяем инвентарь после update
    let world = core.get_world();
    if let Ok(inventory) = world.get::<&nation_of_last_land_core::modules::components::Inventory>(unit_entity) {
        println!("Inventory items after update: {:?}", inventory.0.len());
        for (i, item) in inventory.0.iter().enumerate() {
            println!("  Item {}: {:?}", i, item);
        }
    } else {
        println!("No inventory component");
    }

    // Проверяем attached items
    if let Ok(attached) = world.get::<&nation_of_last_land_core::modules::components::AttachedItems>(unit_entity) {
        println!("Attached items: {:?}", attached.0.len());
        for (slot, item) in attached.0.iter() {
            println!("  Slot {}: {:?}", slot, item);
        }
    } else {
        println!("No attached items component");
    }

    // Теперь удаляем юнит
    use nation_of_last_land_core::world_utils::remove_entity;
    let mut world = core.get_world();
    remove_entity(&mut world, unit_entity).unwrap();

    // Проверяем, остались ли предметы
    let world = core.get_world();
    let item_count = world.query::<&nation_of_last_land_core::modules::markers::Item>().iter().count();
    println!("Remaining items in world: {}", item_count);
}
