use super::weapon_mode::WeaponMode;

#[derive(Clone, Debug)]
pub struct WeaponType {
    pub modes: Vec<WeaponMode>,
}
