use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::markers::IsDead;

fn main() {
    let mut core = Core::new(false);

    println!("Initial state reputation: {}", core.export_world(false).split(',').find(|s| s.contains("reputation")).unwrap_or("not found"));

    // Create a trash
    let trash_entity = core.create_trash().unwrap();
    println!("Created trash entity: {:?}", trash_entity);

    // Mark it as dead
    {
        let world = core.get_world();
        world.insert_one(trash_entity, IsDead {}).unwrap();
    }

    core.update(15.0).unwrap();

    println!("State reputation after trash death: {}", core.export_world(false).split(',').find(|s| s.contains("reputation")).unwrap_or("not found"));

    // Create a vehicle
    let vehicle_entity = core.create_vehicle("VEHICLE_CAR", nation_of_last_land_core::modules::components::Pos { x: 10.0, y: 10.0 }, nation_of_last_land_core::modules::components::Fraction::Neutral).unwrap();
    println!("Created vehicle entity: {:?}", vehicle_entity);

    // Mark vehicle as dead
    {
        let world = core.get_world();
        world.insert_one(vehicle_entity, IsDead {}).unwrap();
    }

    core.update(15.0).unwrap();

    println!("State reputation after vehicle death: {}", core.export_world(false).split(',').find(|s| s.contains("reputation")).unwrap_or("not found"));

    let s = core.export_world(true);
    println!("Final world: {s}");

    // for _i in 0..100 {
    //     core.update(15.0).unwrap();
    //     let s = core.export_world();
    //     println!("Export world: {s}");
    // }
}
