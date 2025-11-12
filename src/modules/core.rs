use crate::modules::manager_create::ManagerCreate;

pub struct Core {
    manager_create: ManagerCreate,
}

impl Core {
    pub fn new() -> Self {
        let manager_create = ManagerCreate::new();
        Core { manager_create }
    }

    pub fn get_manager_create(&self) -> &ManagerCreate {
        &self.manager_create
    }
}
