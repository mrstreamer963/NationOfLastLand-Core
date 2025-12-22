use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Fraction, Pos, Target, Health};
use nation_of_last_land_core::modules::markers::{Unit, IsWaitingTarget, Floor};

fn main() {
    let mut core = Core::new(false);

    // Create a damaged unit of Red faction
    let unit_red = core.create_unit("UNIT_TRASH", Pos { x: 0.0, y: 0.0 }, Fraction::Red).unwrap();

    // Damage the unit by reducing health
    let world = core.get_world();
    if let Ok(mut health) = world.get::<&mut Health>(unit_red) {
        println!("Unit health before damage: current={}, max={}", health.current, health.max);
        health.current = health.max - 0.5; // Make it damaged
        println!("Unit health after damage: current={}, max={}", health.current, health.max);
    }

    // Add IsWaitingTarget to the unit
    world.insert_one(unit_red, IsWaitingTarget {}).unwrap();

    // Create a floor of Red faction nearby
    let floor_red = core.create_floor("FLOOR_PARK", Pos { x: 1.0, y: 1.0 }, Fraction::Red).unwrap();

    // Create an enemy alert far away
    let alert_enemy = core.create_trash().unwrap();
    let world = core.get_world();
    if let Ok(mut pos) = world.get::<&mut Pos>(alert_enemy) {
        pos.x = 100.0;
        pos.y = 100.0;
    }

    println!("Created damaged unit: {:?}", unit_red);
    println!("Created allied floor: {:?}", floor_red);
    println!("Created enemy alert: {:?}", alert_enemy);

    // Check floor has Floor marker
    let has_floor = world.get::<&Floor>(floor_red).is_ok();
    println!("Floor has Floor marker: {}", has_floor);

    // Check floor has Fraction
    if let Ok(fraction) = world.get::<&Fraction>(floor_red) {
        println!("Floor fraction: {:?}", fraction);
    }

    // Check unit fraction
    if let Ok(fraction) = world.get::<&Fraction>(unit_red) {
        println!("Unit fraction: {:?}", fraction);
    }

    // Check before update
    let world = core.get_world();
    let has_waiting = world.get::<&IsWaitingTarget>(unit_red).is_ok();
    println!("Unit has IsWaitingTarget: {}", has_waiting);

    // Update to let AI assign targets
    core.update(15.0).unwrap();

    let world = core.get_world();

    println!("Targets assigned:");
    for (entity, (target, _unit)) in world.query::<(&Target, &Unit)>().iter() {
        println!("  Unit {:?} targets {:?}", entity, target.e);
        if entity == unit_red {
            if target.e == floor_red {
                println!("  SUCCESS: Damaged unit targets allied floor!");
            } else if target.e == alert_enemy {
                println!("  FAIL: Damaged unit targets enemy alert instead of floor!");
            } else {
                println!("  UNKNOWN: Damaged unit targets something else");
            }
        }
    }

    // Check if unit still has IsWaitingTarget
    let has_waiting_after = world.get::<&IsWaitingTarget>(unit_red).is_ok();
    println!("Unit has IsWaitingTarget after update: {}", has_waiting_after);
}
