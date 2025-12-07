use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Pos, BaseType, Guid};
use nation_of_last_land_core::modules::markers::Trash;

fn main() {
    let mut core = Core::new();

    // Create a vehicle
    let vehicle = core.create_vehicle_from_yaml("VEHICLE_CAR", Pos { x: 0.0, y: 0.0 }).unwrap();

    // Create an item with interactions
    let item = core.create_item_from_yaml("ITEM_CLEANER", Pos { x: 0.0, y: 0.0 }).unwrap();

    // Attach item to vehicle
    core.attach(vehicle, item, "front_left").unwrap();


    let w = core.get_world();

    // Create a trash entity close to the vehicle (at position 0.1, 0.1)
    // let _trash = spawn_entity(w, (
    //     BaseType("TRASH".to_string()),
    //     Pos { x: 0.1, y: 0.1 },
    //     Trash {},
    //     Guid::new(),
    // ));

    core.update(15.0).unwrap();

    // Update the world until the vehicle attacks
    println!("Updating world to simulate vehicle movement and attacks:");
    for i in 1..=10 {
        println!("Update {}", i);
        core.update(15.0).unwrap();

        // Get and display the results of attack_vehicles
        // let attack_events = core.get_attack_events();

        // println!("  Attack Events Count: {}", attack_events.len());
        // for event in &attack_events {
        //     println!("    Weapon Mode: {:?}", event.weapon_mode);
        //     println!("    Target Entity: {:?}", event.target_unit);
        // }
    }
}
