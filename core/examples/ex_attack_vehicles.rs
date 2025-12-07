use nation_of_last_land_core::Core;
use nation_of_last_land_core::modules::components::{Health, Pos};
use nation_of_last_land_core::modules::markers::{Alert, AttackEvent, IsTargetNear, IsWaitingTarget, Vehicle};


fn main() {
    let mut core = Core::new();

    // Update the world until the vehicle attacks
    println!("Updating world to simulate vehicle movement and attacks:");
    for i in 1..=100 {
        println!("Update {}", i);
        core.update(15.0).unwrap();

        let w = core.get_world();

        // println!("Vehicles positions:");
        // for (_entity, (pos, _vehicle, waiting, target_near)) in w.query::<(&Pos, &Vehicle, Option<&IsWaitingTarget>, 
        //     Option<&IsTargetNear>)>().iter() {            
        //     let waiting_str = if waiting.is_some() { "present" } else { "absent" };
        //     let target_near = if target_near.is_some() { "present" } else { "absent" };
        //     println!("  {:?}; IsWaitingTarget: {}, IsTargetNear: {}", pos, waiting_str, target_near);
        // }

        println!("Alerts:");
        for (entity, (pos, _alert, health)) in w.query::<(&Pos, &Alert, &Health)>().iter() {
            println!("  Entity: {:?}; Pos: {:?} Health: {:?}", entity, pos, health.current);
        }

        println!("AttackEvents:");
        for (entity, _attack_event) in w.query::<&AttackEvent>().iter() {
            println!("  Entity: {:?}", entity);
        }
    }
}
