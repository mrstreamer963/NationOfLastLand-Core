use crate::modules::components::Pos;
use crate::modules::components::{MaxSpeed, Velocity, Target};
use crate::modules::markers::{IsMoving, IsTargetNear};
use crate::modules::setup::Spatial;
use hecs::World;

pub fn do_move(world: &mut World, spatial: &Spatial) {
    let mut entities_to_stop = Vec::new();

    for (entity, (pos, target, velocity, max_speed, _)) in world
        .query::<(&mut Pos, &Target, &mut Velocity, &MaxSpeed, &IsMoving)>()
        .iter()
    {
        // Find target position directly by Entity
        let target_entity = target.0;
        if let Ok(mut query) = world.query_one::<(&Pos,)>(target_entity) {
            let (target_pos,) = query.get().unwrap();
            let dx = target_pos.x - pos.x;
            let dy = target_pos.y - pos.y;
            let distance_squared = dx * dx + dy * dy;

            // Threshold to consider reached, e.g., 1.0 units
            if distance_squared < spatial.threshold * spatial.threshold {
                // Arrived at target: set position to target and reset velocity to zero
                *pos = *target_pos;
                *velocity = Velocity { x: 0.0, y: 0.0 };
                entities_to_stop.push(entity);
            } else {
                // Move towards target: compute direction and set velocity
                let distance = distance_squared.sqrt();
                let dir_x = dx / distance;
                let dir_y = dy / distance;
                let new_vel_x = dir_x * max_speed.0;
                let new_vel_y = dir_y * max_speed.0;
                velocity.x = new_vel_x;
                velocity.y = new_vel_y;
                pos.x += new_vel_x;
                pos.y += new_vel_y;
            }
        }
    }

    // Change markers for stopped vehicles
    for entity in entities_to_stop {
        world.insert_one(entity, IsTargetNear {}).unwrap();
        world.remove_one::<IsMoving>(entity).unwrap();
    }
}
