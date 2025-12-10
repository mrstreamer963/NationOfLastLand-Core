use crate::modules::components::Pos;
use crate::modules::components::{MaxSpeed, Velocity, Target};
use crate::modules::markers::{IsMoving, IsTargetNear, IsWaitingTarget};
use crate::modules::setup::Spatial;
use hecs::World;

pub fn set_speed_by_target(world: &mut World, spatial: &Spatial) {
    // Collect all entities that need speed setting
    let mut entities_near_target = Vec::new();
    let mut entities_missing_target = Vec::new();

    for (entity, (pos, velocity, target, max_speed, _moving)) in
        world.query::<(&mut Pos, &mut Velocity, &Target, &MaxSpeed, &IsMoving)>().iter()
    {
        // Get target position by querying directly
        if let Ok(mut query) = world.query_one::<(&Pos,)>(target.e) {
            if let Some((target_pos,)) = query.get() {
                let dx = target_pos.x - pos.x;
                let dy = target_pos.y - pos.y;
                let dist_sq = dx * dx + dy * dy;

                let speed = max_speed.0.min;

                let desired_vx = dx;
                let desired_vy = dy;
                let desired_speed_sq = desired_vx.powi(2) + desired_vy.powi(2);

                if dist_sq < spatial.threshold * spatial.threshold {
                    // Close enough to target
                    velocity.x = 0.0;
                    velocity.y = 0.0;

                    *pos = *target_pos;

                    entities_near_target.push(entity);
                } else if desired_speed_sq == 0.0 {
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                } else if desired_speed_sq > speed.powi(2) {
                    let scale = speed / desired_speed_sq.sqrt();
                    velocity.x = desired_vx * scale;
                    velocity.y = desired_vy * scale;
                } else {
                    // Distance <= speed, reach exactly
                    velocity.x = desired_vx;
                    velocity.y = desired_vy;
                }
            } else {
                // Target does not exist, mark for removal
                entities_missing_target.push(entity);
            }
        } else {
            // If query_one fails, also consider target missing
            entities_missing_target.push(entity);
        }
    }

    // Set IsTargetNear for close entities and remove IsMoving
    for entity in entities_near_target {
        world.insert_one(entity, IsTargetNear {}).unwrap();
        world.remove_one::<IsMoving>(entity).unwrap();
    }

    // Remove IsMoving and set IsWaitingTarget for entities with missing targets
    for entity in entities_missing_target {
        world.remove_one::<IsMoving>(entity).unwrap();
        world.insert_one(entity, IsWaitingTarget {}).unwrap();
    }
}

pub fn do_move(world: &mut World, _spatial: &Spatial) {
    for (_entity, (pos, velocity, _)) in world
        .query::<(&mut Pos, &mut Velocity, &IsMoving)>()
        .iter()
    {
        pos.x += velocity.x;
        pos.y += velocity.y;
    }
}
