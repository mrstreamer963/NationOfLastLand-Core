use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::markers::IsDead;

fn main() {
    let mut core = Core::new();

    // Create a trash
    let trash_entity = core.create_trash().unwrap();
    println!("Created trash entity: {:?}", trash_entity);

    // Mark it as dead
    let world = core.get_world();
    world.insert_one(trash_entity, IsDead {}).unwrap();

    println!("State reputation before update: {}", core.export_world(false).split(',').find(|s| s.contains("reputation")).unwrap_or("not found"));

    core.update(15.0).unwrap();

    println!("State reputation after update: {}", core.export_world(false).split(',').find(|s| s.contains("reputation")).unwrap_or("not found"));

    let s = core.export_world(true);
    println!("Export world: {s}");

    // for _i in 0..100 {
    //     core.update(15.0).unwrap();
    //     let s = core.export_world();
    //     println!("Export world: {s}");
    // }
}
