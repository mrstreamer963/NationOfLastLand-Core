use crate::modules::components::pos::Pos;

pub struct ManagerCreate {}

impl ManagerCreate {
    pub fn new() -> Self {
        ManagerCreate {}
    }

    pub fn create_waste(&self, pos: Pos) -> Result<(), String> {
        Ok(())
    }
}
