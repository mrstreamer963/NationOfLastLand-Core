use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Health, Pos, Fraction};
use nation_of_last_land_core::modules::markers::IsWaitingTarget;

fn main() {
    let mut core = Core::new(false);

    // Create UNIT_WASTE with weapons
    let waste = core.create_unit("UNIT_WASTE", Pos { x: 0.0, y: 0.0 }, Fraction::Red).unwrap();

    // Create VEHICLE_CAR as target (close enough to attack)
    let car = core.create_vehicle("VEHICLE_CAR", Pos { x: 0.05, y: 0.05 }, Fraction::Neutral).unwrap();

    // Check if UNIT_WASTE has IsWaitingTarget
    let world = core.get_world();
    let has_waiting_target = world.get::<&IsWaitingTarget>(waste).is_ok();
    println!("UNIT_WASTE has IsWaitingTarget: {}", has_waiting_target);

    // Get initial health of car
    let initial_health = if let Ok(health) = world.get::<&Health>(car) {
        health.current
    } else {
        0.0
    };
    println!("Initial VEHICLE_CAR health: {}", initial_health);

    // Update world multiple times
    for i in 1..=10 {
        println!("----- Update {}", i);
        core.update(15.0).unwrap();

        let world = core.get_world();

        // Check health of car
        if let Ok(health) = world.get::<&Health>(car) {
            println!("VEHICLE_CAR health: {}", health.current);
        }

        // Check if waste is moving or near target
        let is_moving = world.get::<&nation_of_last_land_core::modules::markers::IsMoving>(waste).is_ok();
        let is_near_target = world.get::<&nation_of_last_land_core::modules::markers::IsTargetNear>(waste).is_ok();
        println!("UNIT_WASTE - IsMoving: {}, IsTargetNear: {}", is_moving, is_near_target);
    }
}
