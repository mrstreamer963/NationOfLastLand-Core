use super::weapon_mode::WeaponMode;

#[derive(Clone, Debug)]
pub struct Weapon {
    pub modes: Vec<WeaponMode>,
}
