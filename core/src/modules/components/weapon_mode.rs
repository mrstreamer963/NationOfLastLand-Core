use super::damage_type::DamageType;

#[derive(Clone, Copy, Debug)]
pub struct WeaponMode {
    pub damage_type: DamageType,
    pub damage: i32,
    pub range: f32,
}
