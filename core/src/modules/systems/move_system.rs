use crate::modules::components::Pos;
use crate::modules::components::{MaxSpeed, Velocity, Target};
use crate::modules::markers::{IsMoving, IsTargetNear, IsWaitingTarget};
use crate::modules::setup::Spatial;
use hecs::World;

enum Action {
    NearTarget { entity: hecs::Entity, target_pos: Pos },
    SetVelocity { entity: hecs::Entity, vx: f32, vy: f32 },
    MissingTarget { entity: hecs::Entity },
}

pub fn set_speed_by_target(world: &mut World, spatial: &Spatial) {
    // Collect actions to perform
    let mut actions = Vec::new();

    for (entity, (pos, target, _moving)) in
        world.query::<(&Pos, &Target, &IsMoving)>().iter()
    {
        // Get target position by querying directly
        if let Ok(mut query) = world.query_one::<(&Pos,)>(target.e) {
            if let Some((target_pos,)) = query.get() {
                let dx = target_pos.x - pos.x;
                let dy = target_pos.y - pos.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < spatial.threshold * spatial.threshold {
                    // Close enough to target
                    actions.push(Action::NearTarget { entity, target_pos: *target_pos });
                } else {
                    // Check if entity has MaxSpeed
                    if let Ok(max_speed) = world.get::<&MaxSpeed>(entity) {
                        let speed = max_speed.0.min;

                        let desired_vx = dx;
                        let desired_vy = dy;
                        let desired_speed_sq = desired_vx.powi(2) + desired_vy.powi(2);

                        if desired_speed_sq == 0.0 {
                            actions.push(Action::SetVelocity { entity, vx: 0.0, vy: 0.0 });
                        } else if desired_speed_sq > speed.powi(2) {
                            let scale = speed / desired_speed_sq.sqrt();
                            actions.push(Action::SetVelocity { entity, vx: desired_vx * scale, vy: desired_vy * scale });
                        } else {
                            // Distance <= speed, reach exactly
                            actions.push(Action::SetVelocity { entity, vx: desired_vx, vy: desired_vy });
                        }
                    }
                    // If no MaxSpeed, don't set velocity, but still check distance for NearTarget
                }
            } else {
                // Target does not exist, mark for removal
                actions.push(Action::MissingTarget { entity });
            }
        } else {
            // If query_one fails, also consider target missing
            actions.push(Action::MissingTarget { entity });
        }
    }

    // Apply actions
    for action in actions {
        match action {
            Action::NearTarget { entity, target_pos } => {
                if let Ok(mut pos) = world.get::<&mut Pos>(entity) {
                    *pos = target_pos;
                }
                if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
                world.insert_one(entity, IsTargetNear {}).unwrap();
                world.remove_one::<IsMoving>(entity).unwrap();
            }
            Action::SetVelocity { entity, vx, vy } => {
                if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
                    velocity.x = vx;
                    velocity.y = vy;
                }
            }
            Action::MissingTarget { entity } => {
                world.remove_one::<IsMoving>(entity).unwrap();
                world.insert_one(entity, IsWaitingTarget {}).unwrap();
            }
        }
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
