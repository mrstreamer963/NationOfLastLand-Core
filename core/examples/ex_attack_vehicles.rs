use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::Pos;
use nation_of_last_land_core::modules::markers::{Alert, IsTargetNear, IsWaitingTarget, Vehicle};
use hecs::Entity;

fn main() {
    let mut core = Core::new();

    // Create a vehicle
    // let vehicle = core.create_vehicle_from_yaml("VEHICLE_CAR", Pos { x: 0.0, y: 0.0 }).unwrap();

    // Create an item with interactions
    // let item = core.create_item_from_yaml("ITEM_CLEANER", Pos { x: 0.0, y: 0.0 }).unwrap();

    // Attach item to vehicle
    // core.attach(vehicle, item, "front_left").unwrap();


    // Update the world until the vehicle attacks
    println!("Updating world to simulate vehicle movement and attacks:");
    for i in 1..=100 {
        println!("Update {}", i);
        core.update(15.0).unwrap();

        let w = core.get_world();
        println!("Vehicles positions:");
        for (_entity, (pos, _vehicle, waiting, target_near)) in w.query::<(&Pos, &Vehicle, Option<&IsWaitingTarget>, 
            Option<&IsTargetNear>)>().iter() {            
            let waiting_str = if waiting.is_some() { "present" } else { "absent" };
            let target_near = if target_near.is_some() { "present" } else { "absent" };
            println!("  {:?}; IsWaitingTarget: {}, IsTargetNear: {}", pos, waiting_str, target_near);
        }

        println!("Alerts:");
        for (entity, (pos, _alert)) in w.query::<(&Pos, &Alert)>().iter() {
            println!("  Entity: {:?}; Pos: {:?}", entity, pos);
        }
    }
}
