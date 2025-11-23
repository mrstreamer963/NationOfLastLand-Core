mod max_speed;
mod pos;
mod reputation;
mod rot;
mod states;
mod target;
mod toxic_power;
mod velocity;
mod health;
mod clean_power;

pub use max_speed::MaxSpeed;
pub use pos::Pos;
pub use reputation::Reputation;
pub use rot::Rot;
pub use states::{IsMoving, IsStopped, IsWaitingTarget};
pub use target::Target;
pub use toxic_power::ToxicPower;
pub use velocity::Velocity;
pub use health::Health;
pub use clean_power::CleanPower;

