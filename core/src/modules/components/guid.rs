use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq, Eq, Hash)]
pub struct Guid(pub Uuid);

impl Guid {
    pub fn new() -> Self {
        Guid(Uuid::new_v4())
    }
}
