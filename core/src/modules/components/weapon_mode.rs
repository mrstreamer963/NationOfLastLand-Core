use super::damage_type::DamageType;

#[derive(Clone, Debug)]
pub struct WeaponMode {
    pub damage_type: DamageType,
    pub damage: f32,
    pub range: f32,
}
