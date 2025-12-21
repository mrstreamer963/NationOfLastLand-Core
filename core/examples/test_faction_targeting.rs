use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Fraction, Pos, Target};
use nation_of_last_land_core::modules::markers::{Unit, IsWaitingTarget};

fn main() {
    let mut core = Core::new(false);

    // Create units of different factions
    let unit_red1 = core.create_unit("UNIT_TRASH", Pos { x: 0.0, y: 0.0 }, Fraction::Red).unwrap();
    let unit_red2 = core.create_unit("UNIT_TRASH", Pos { x: 1.0, y: 1.0 }, Fraction::Red).unwrap();
    let unit_blue = core.create_unit("UNIT_TRASH", Pos { x: 2.0, y: 2.0 }, Fraction::Blue).unwrap();

    // Add IsWaitingTarget to units
    let world = core.get_world();
    world.insert_one(unit_red1, IsWaitingTarget {}).unwrap();
    world.insert_one(unit_red2, IsWaitingTarget {}).unwrap();
    world.insert_one(unit_blue, IsWaitingTarget {}).unwrap();

    // Create alerts as targets at specific positions
    let alert1 = core.create_trash().unwrap();
    let alert2 = core.create_waste().unwrap();

    // Set alert positions closer
    let world = core.get_world();
    if let Ok(mut pos) = world.get::<&mut Pos>(alert1) {
        pos.x = 10.0;
        pos.y = 10.0;
    }
    if let Ok(mut pos) = world.get::<&mut Pos>(alert2) {
        pos.x = 11.0;
        pos.y = 11.0;
    }

    println!("Created units:");
    println!("  Red1: {:?}", unit_red1);
    println!("  Red2: {:?}", unit_red2);
    println!("  Blue: {:?}", unit_blue);
    println!("Created alerts:");
    println!("  Trash: {:?}", alert1);
    println!("  Waste: {:?}", alert2);

    // Update a few times to let AI assign targets
    for i in 1..=5 {
        println!("----- Update {}", i);
        core.update(15.0).unwrap();

        let world = core.get_world();

        println!("Targets assigned:");
        for (entity, (target, _unit)) in world.query::<(&Target, &Unit)>().iter() {
            println!("  Unit {:?} targets {:?}", entity, target.e);
        }
    }
}
