
mod spatial;
mod reputation;
mod states;
mod health;
mod target_id;
mod alert_type;
mod damage_type;

pub use spatial::{MaxSpeed, Pos, Rot, Velocity, TargetPos};
pub use reputation::Reputation;
pub use states::{IsMoving, IsStopped, IsWaitingTarget};
pub use health::Health;
pub use target_id::TargetId;
pub use alert_type::AlertType;
pub use damage_type::DamageType;
